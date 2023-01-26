import fs from 'fs'
import { promisify } from 'util'
import { createFakeFolder, removeFolderToTests } from "./test-tools";
import { createFolderToTests } from "./test-tools/createFolderToTests";
import { AcceptableError, Bash } from "./tools/Bash";

const FOLDER_TO_TESTS = process.env.FOLDER_TO_TESTS
if (FOLDER_TO_TESTS === undefined) {
  throw Error('Defina a variável FOLDER_TO_TESTS no arquivo .env.tests')
}
const PROJECT_FOLDER = process.env.PROJECT_FOLDER
if (PROJECT_FOLDER === undefined) {
  throw Error('Defina a variável PROJECT_FOLDER no arquivo .env.tests')
}

function makeSut() {
  return new Bash(`${PROJECT_FOLDER}/src/start.sh`)
}

interface Parameters {
  dotenvType?: string
  dotenvContent: string
  foldersListContent: string
}
async function mountParameters(parameters: Parameters) {
  const { dotenvType, dotenvContent, foldersListContent } = parameters
  const path = `${FOLDER_TO_TESTS}/src/tools`
  await promisify(fs.mkdir)(path, { recursive: true })
  await promisify(fs.copyFile)(`${PROJECT_FOLDER}/src/tools/check.sh`, `${path}/check.sh`)
  await promisify(fs.copyFile)(`${PROJECT_FOLDER}/src/tools/dotenv.sh`, `${path}/dotenv.sh`)
  await promisify(fs.copyFile)(`${PROJECT_FOLDER}/src/tools/validator.sh`, `${path}/validator.sh`)
  await promisify(fs.copyFile)(`${PROJECT_FOLDER}/src/main.sh`, `${FOLDER_TO_TESTS}/src/main.sh`)
  await promisify(fs.writeFile)(`${FOLDER_TO_TESTS}/.env${dotenvType || ''}`, dotenvContent)
  await promisify(fs.writeFile)(`${FOLDER_TO_TESTS}/folders_list.txt`, foldersListContent)
}

function sleep(seconds: number) {
  return new Promise((resolve) => {
    setTimeout(resolve, seconds * 1000)
  })
}

describe('start.sh', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve encerrar o script com erro se não conseguir encontrar o script de verificação', async () => {
    const sut = makeSut()
    const expectedError = 'A pasta do projeto definida não está correta. (404)'
    await expect(sut.run('/any/folder')).rejects.toThrowError(expectedError)
  })

  test('deve encerrar o script com erro se o retorno do script de verificação não estiver correto', async () => {
    const sut = makeSut()
    const path = `${FOLDER_TO_TESTS}/src/tools/`
    await promisify(fs.mkdir)(path, { recursive: true })
    await promisify(fs.writeFile)(`${path}/check.sh`, '#!/bin/bash\necho "any"')
    const expectedError = 'A pasta do projeto definida não está correta. (400)'
    await expect(sut.run(FOLDER_TO_TESTS)).rejects.toThrowError(expectedError)
  })

  test('deve criar o arquivo de logs', async () => {
    const sut = makeSut()
    await mountParameters({ dotenvContent: 'USER_FOLDERS_LIST_PATH="any"', foldersListContent: '' })
    try {
      await sut.run(FOLDER_TO_TESTS)
    } catch (_) {}
    const files = await promisify(fs.readdir)(FOLDER_TO_TESTS)
    expect(files).toContain('logs.txt')
  })

  test('deve carregar as variáveis de ambiente usando dotenv.sh', async () => {
    const sut = makeSut()
    const dotenvContent = `ANY_KEY="ANY_VALUE"\nUSER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`
    await mountParameters({
      foldersListContent: `${FOLDER_TO_TESTS}/\n- 10 zip`,
      dotenvContent
    })
    const result = await sut.run(`${FOLDER_TO_TESTS} any dotenv`)
    expect(result).toMatch(/^ANY_KEY="ANY_VALUE"/)
  })

  test('deve carregar as variáveis de .env.dev usando dotenv.sh', async () => {
    const sut = makeSut()
    const dotenvContent = `DEV_KEY="DEV_VALUE"\nUSER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`
    await mountParameters({
      foldersListContent: `${FOLDER_TO_TESTS}/\n- 10 zip`,
      dotenvContent, dotenvType: '.dev'
    })
    const result = await sut.run(`${FOLDER_TO_TESTS} dev dotenv`)
    expect(result).toMatch(/^DEV_KEY="DEV_VALUE"/)
  })

  test('deve encerrar o script com erro se o caminho para a lista de pastas do usuário não for informado', async () => {
    const sut = makeSut()
    await mountParameters({ dotenvContent: '', foldersListContent: '' })
    const expectedError = 'não foi informado no arquivo .env o caminho para a lista de pastas do usuário'
    await expect(sut.run(FOLDER_TO_TESTS)).rejects.toThrowError(expectedError)
  })

  test('deve encerrar com erro 4 se o arquivo do usuário for inválido e existir arquivo já validado', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await promisify(fs.writeFile)(`${FOLDER_TO_TESTS}/valid-list-of-folders`, `${fakeFolder}\n- 10 zip`)
    await sleep(1)
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: '/any/folder\n- 10 zip'
    })
    await expect(sut.run(FOLDER_TO_TESTS)).rejects.toThrowError(AcceptableError)
  })
})

describe('\n  Main.sh', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve registrar nos logs o inicio da execução do script main', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: `${fakeFolder}\n- 10 zip`
    })
    await sut.run(`${FOLDER_TO_TESTS}`)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs).toContain('[Status] iniciando execução do SCAR usando o arquivo validado')
  })

  test('deve encerrar script registrando nos logs se main.sh gerar algum erro', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: `${fakeFolder}\n- 10 zip`
    })
    await expect(sut.run(`${FOLDER_TO_TESTS} any simulate_error`))
      .rejects.toThrowError('[Error(1): main.sh] Simulated error\n')
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs).toContain('[Error(1): main.sh] Simulated error')
  })

  test('deve executar main.sh sem gerar erros se todos os parâmetros estiverem corretos', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: `${fakeFolder}\n- 10 zip`
    })
    await expect(sut.run(`${FOLDER_TO_TESTS}`)).resolves.not.toThrow()
  })
})

/* - CASO 01
verifica se arquivo validado existe (NÃO)
LOG: Validando arquivo "PATH DO ARQUIVO DO USUÁRIO"...
verifica se arquivo do usuário é válido (NÃO)
ENCERRA A APLICAÇÃO COM ERRO */
describe('\n  Caso 01 (não existe arquivo validado e arquivo do usuário é inválido', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve registrar nos logs se não existir arquivo validado e que tentará validar a lista do usuário', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: `${fakeFolder}\n- 10 zip`
    })
    await sut.run(FOLDER_TO_TESTS)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs)
      .toContain('[Warning] Validador -> Não foi encontrado um arquivo já validado, tentando validar o arquivo do usuário...')
  })

  test('deve registrar nos logs e encerrar script com erro se a lista de pastas do usuário for inválida', async () => {
    const sut = makeSut()
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: '/any/folder\n- 10 zip'
    })
    const expectedError = `O arquivo "${FOLDER_TO_TESTS}/folders_list.txt" é inválido e não existe arquivo validado anteriormente`
    await expect(sut.run(FOLDER_TO_TESTS)).rejects.toThrowError(expectedError)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs)
      .toContain('[Error] validator -> Uma pasta informada no arquivo não foi encontrada')
    expect(logs)
      .toContain(`[FatalError] Validador -> ${expectedError}`)
  })
})

/* - CASO 02
verifica se arquivo validado existe (NÃO)
LOG: Validando arquivo "PATH DO ARQUIVO DO USUÁRIO"...
verifica se arquivo do usuário é válido (SIM)
faz cópia do arquivo do usuário para pasta do projeto - validator
LOG: Arquivo "PATH DO ARQUIVO DO USUÁRIO" validado com sucesso, executando SCAR
EXECUTA SCAR COM ARQUIVO VALIDADO NA PASTA DO PROJETO */
describe('\n  Caso 02 (não existe arquivo validado e arquivo do usuário é válido', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve registrar nos logs se o arquivo do usuário for válido', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: `${fakeFolder}\n- 10 zip`
    })
    await sut.run(FOLDER_TO_TESTS)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs)
      .toContain('[Success] Validator -> Arquivo do usuário validado com sucesso')
  })

  test('deve encerrar o script sem erro se o arquivo do usuário for válido', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: `${fakeFolder}\n- 10 zip`
    })
    await expect(sut.run(FOLDER_TO_TESTS)).resolves.not.toThrow()
  })
})

/* - CASO 03
verifica se arquivo validado existe (SIM)
verifica se houve alterações desde a ultima validação (NÃO)
LOG: Arquivo "PATH DO ARQUIVO DO USUÁRIO" não foi alterado recentemente, executando SCAR
EXECUTA SCAR COM ARQUIVO VALIDADO NA PASTA DO PROJETO */
describe('\n  Caso 03 (exite arquivo validado e não houve alterações no arquivo do usuário', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve registrar nos logs que não houve alterações no arquivo do usuário', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: `${fakeFolder}\n- 10 zip`
    })
    await sleep(1)
    await promisify(fs.writeFile)(`${FOLDER_TO_TESTS}/valid-list-of-folders`, `${fakeFolder}\n- 10 zip`)
    await sut.run(FOLDER_TO_TESTS)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs)
      .toContain('[Status] Validador -> O arquivo do usuário não foi alterado desde a ultima validação')
  })
})

/* - CASO 04
verifica se arquivo validado existe (SIM)
verifica se houve alterações desde a ultima validação (SIM)
LOG: Validando arquivo "PATH DO ARQUIVO DO USUÁRIO"...
verifica se arquivo do usuário é válido (NÃO)
LOG: Arquivo "PATH DO ARQUIVO DO USUÁRIO" é inválido, executando SCAR com ultimo arquivo válido
EXECUTA SCAR COM ARQUIVO VALIDADO NA PASTA DO PROJETO */
describe('\n  Caso 04 (existe arquivo validado e houve alterações no arquivo do usuário que o deixaram inválido', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve registrar nos logs que houve alterações no arquivo do usuário', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await promisify(fs.writeFile)(`${FOLDER_TO_TESTS}/valid-list-of-folders`, `${fakeFolder}\n- 10 zip`)
    await sleep(1)
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: `${fakeFolder}\n- 10 zip`
    })
    await sut.run(FOLDER_TO_TESTS)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs)
      .toContain('[Status] Validador -> O arquivo do usuário foi alterado, iniciando validação...')
  })

  test('deve registrar nos logs o erro do arquivo do usuário e informar que irá usar arquivo já validado', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await promisify(fs.writeFile)(`${FOLDER_TO_TESTS}/valid-list-of-folders`, `${fakeFolder}\n- 10 zip`)
    await sleep(1)
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: '/any/folder\n- 10 zip'
    })
    try {
      await sut.run(FOLDER_TO_TESTS)
    } catch (error) {}
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs)
      .toContain('[Error] validator -> Uma pasta informada no arquivo não foi encontrada')
    const expectedError = `O arquivo "${FOLDER_TO_TESTS}/folders_list.txt" é inválido, mas existe arquivo já validado`
    expect(logs)
      .toContain(`[Warning] Validador -> ${expectedError}`)
  })
})

/* - CASO 05
verifica se arquivo validado existe (SIM)
verifica se houve alterações desde a ultima validação (SIM)
LOG: Validando arquivo "PATH DO ARQUIVO DO USUÁRIO"...
verifica se arquivo do usuário é válido (SIM)
LOG: Arquivo "PATH DO ARQUIVO DO USUÁRIO" validado com sucesso, executando SCAR
EXECUTA SCAR COM ARQUIVO VALIDADO NA PASTA DO PROJETO */
describe('\n  Caso 05 (existe arquivo validado, houve alterações no arquivo do usuário e o arquivo é válido', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve registrar nos logs se o arquivo do usuário for válido', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await promisify(fs.writeFile)(`${FOLDER_TO_TESTS}/valid-list-of-folders`, `${fakeFolder}\n- 10 zip`)
    await sleep(1)
    await mountParameters({
      dotenvContent: `USER_FOLDERS_LIST_PATH="${FOLDER_TO_TESTS}/folders_list.txt"`,
      foldersListContent: `${fakeFolder}\n- 10 zip`
    })
    await sut.run(FOLDER_TO_TESTS)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs)
      .toContain('[Success] Validator -> Arquivo do usuário validado com sucesso')
  })
})
