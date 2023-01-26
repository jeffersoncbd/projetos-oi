import fs from 'fs'
import { promisify } from 'util'
import 'dotenv/config'
import { Bash } from "./tools/Bash"
import { createFolderToTests } from './test-tools/createFolderToTests'
import { createFakeFolder, removeFolderToTests } from './test-tools'

const FOLDER_TO_TESTS = process.env.FOLDER_TO_TESTS
if (FOLDER_TO_TESTS === undefined) {
  throw Error('Defina a variável FOLDER_TO_TESTS no arquivo .env.tests')
}
const PROJECT_FOLDER = process.env.PROJECT_FOLDER
if (PROJECT_FOLDER === undefined) {
  throw Error('Defina a variável PROJECT_FOLDER no arquivo .env.tests')
}

function makeSut() {
  return new Bash(`${PROJECT_FOLDER}/src/main.sh`)
}

interface Parameters {
  fileContent: string[]
}
async function mountParameters(parameters: Parameters) {
  const { fileContent } = parameters

  const filePath = `${FOLDER_TO_TESTS}/valid-list-of-folders`
  await promisify(fs.writeFile)(filePath, fileContent.join('\n'))

  return { filePath, parsed: `${FOLDER_TO_TESTS}` }
}

async function createFakeFile(folderPath: string, daysAgo: number, name?: string) {
  const bash = new Bash(`${PROJECT_FOLDER}/src/test-tools/create-fake-file/main.sh`)
  const randomID = Math.round(Math.random() * 100000)
  await bash.run(`${folderPath} ${name || randomID} 1 ${daysAgo}`)
  return `file-${randomID}`
}

describe('main.sh', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve encerrar o script com erro caso o parâmetro $2 (simulação de erro) seja informado', async () => {
    const sut = makeSut()
    const expectedError = 'Simulated error'
    await expect(sut.run('any simulate_error')).rejects.toThrowError(expectedError)
  })

  test('deve encerrar o script com erro caso o parâmetro $1 (lista de pastas) não seja informado', async () => {
    const sut = makeSut()
    const expectedError = 'O caminho para o arquivo com a lista de pastas deve ser informado.'
    await expect(sut.run('')).rejects.toThrowError(expectedError)
  })

  test('deve encerrar o script com erro caso o parâmetro $1 (lista de pastas) não exista', async () => {
    const sut = makeSut()
    const expectedError = 'O caminho para o arquivo com a lista de pastas não foi encontrado.'
    await expect(sut.run('/any/path')).rejects.toThrowError(expectedError)
  })

  test('deve executar os próximos comandos após um zip', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/ZIP/`
    await promisify(fs.mkdir)(fakeFolder)
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip', fakeFolder, '- 20 remove' ]
    })
    const fakeFileToZip = await createFakeFile(fakeFolder, 13)
    await createFakeFile(fakeFolder, 24)
    await sut.run(parameters.parsed)
    const files = await promisify(fs.readdir)(fakeFolder)
    expect(files).toEqual([`${fakeFileToZip}.gz`])
  })


  test('deve registrar nos logs o incio da execução dos comandos', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip' ]
    })
    await sut.run(parameters.parsed)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs).toContain('SCAR -> A lista de pastas foi carregada, iniciando execução de comandos.')
  })

  test('deve registrar no log o inicio de tratamento de uma pasta', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip' ]
    })
    await sut.run(parameters.parsed)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs).toContain(`SCAR -> processando a pasta \"${fakeFolder}\"...`);
  })

  test('deve registrar nos logs os comandos que estão sendo executados', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip - 15 move /tmp - 20 remove' ]
    })
    await sut.run(parameters.parsed)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs).toContain('\n    compactando 0 arquivos: OK\n    movendo 0 arquivos: OK\n    removendo 0 arquivos: OK');
  })

  test('deve registrar nos logs o término do script', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip' ]
    })
    await sut.run(parameters.parsed)
    const logs = (await promisify(fs.readFile)(`${FOLDER_TO_TESTS}/logs.txt`)).toString()
    expect(logs).toContain('SCAR -> Todas as pastas foram processadas! encerrando script...')
  })
})

describe('\n  ZIP files', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve ignorar arquivos recentes que ainda não estão no alcance de execução', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/ZIP/`
    await promisify(fs.mkdir)(fakeFolder)
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip' ]
    })
    const fakeFile = await createFakeFile(fakeFolder, 3)
    await sut.run(parameters.parsed)
    const files = await promisify(fs.readdir)(fakeFolder)
    expect(files).toContain(fakeFile)
  })

  test('deve zipar os arquivos que entraram no alcance para execução', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/ZIP/`
    await promisify(fs.mkdir)(fakeFolder)
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip' ]
    })
    const newFakeFile = await createFakeFile(fakeFolder, 5)
    const fakeFileToZip = await createFakeFile(fakeFolder, 13)
    await sut.run(parameters.parsed)
    const files = await promisify(fs.readdir)(fakeFolder)
    expect(files).toContain(newFakeFile)
    expect(files).toContain(`${fakeFileToZip}.gz`)
  })

  test('se informado a data de exclusão dos arquivos, estes devem ser removidos quando entrarem no alcance', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/ZIP/`
    await promisify(fs.mkdir)(fakeFolder)
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip 20' ]
    })
    const newFakeFile = await createFakeFile(fakeFolder, 6)
    const fakeFileToZip = await createFakeFile(fakeFolder, 13)
    const fakeFileToRemove = await createFakeFile(fakeFolder, 26)
    await sut.run(parameters.parsed)
    const files = await promisify(fs.readdir)(fakeFolder)
    expect(files).toContain(newFakeFile)
    expect(files).toContain(`${fakeFileToZip}.gz`)
    expect(files).not.toContain(fakeFileToRemove)
  })

  test('se existir arquivo zipado com mesmo nome (reenvio), deve ser zipado com o sufixo "N_reenvio"', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/ZIP/`
    await promisify(fs.mkdir)(fakeFolder)
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip' ]
    })
    await createFakeFile(fakeFolder, 15, 'any_name.gz')
    await createFakeFile(fakeFolder, 15, 'any_name')
    await sut.run(parameters.parsed)
    const files = await promisify(fs.readdir)(fakeFolder)
    expect(files).toContain('file-any_name.gz')
    expect(files).toContain('file-any_name-1o_reenvio.gz')
  })

  test('se existir reenvios zipados, deve continuar zipando na ordem', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/ZIP/`
    await promisify(fs.mkdir)(fakeFolder)
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip' ]
    })
    await createFakeFile(fakeFolder, 15, 'any_name-1o_reenvio.gz')
    await createFakeFile(fakeFolder, 15, 'any_name.gz')
    await createFakeFile(fakeFolder, 15, 'any_name')
    await sut.run(parameters.parsed)
    const files = await promisify(fs.readdir)(fakeFolder)
    expect(files).toContain('file-any_name.gz')
    expect(files).toContain('file-any_name-1o_reenvio.gz')
    expect(files).toContain('file-any_name-2o_reenvio.gz')
  })

  test('se existir reenvios zipados, deve continuar zipando na ordem (2)', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/ZIP/`
    await promisify(fs.mkdir)(fakeFolder)
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, '- 10 zip' ]
    })
    await createFakeFile(fakeFolder, 15, 'any_name-4o_reenvio.gz')
    await createFakeFile(fakeFolder, 15, 'any_name-3o_reenvio.gz')
    await createFakeFile(fakeFolder, 15, 'any_name-2o_reenvio.gz')
    await createFakeFile(fakeFolder, 15, 'any_name-1o_reenvio.gz')
    await createFakeFile(fakeFolder, 15, 'any_name.gz')
    await createFakeFile(fakeFolder, 15, 'any_name')
    await sut.run(parameters.parsed)
    const files = await promisify(fs.readdir)(fakeFolder)
    expect(files).toContain('file-any_name.gz')
    expect(files).toContain('file-any_name-1o_reenvio.gz')
    expect(files).toContain('file-any_name-2o_reenvio.gz')
    expect(files).toContain('file-any_name-3o_reenvio.gz')
    expect(files).toContain('file-any_name-4o_reenvio.gz')
    expect(files).toContain('file-any_name-5o_reenvio.gz')
  })
})

describe('\n  MOVE files', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve ignorar arquivos recentes que ainda não estão no alcance de execução', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/MOVE/`
    await promisify(fs.mkdir)(fakeFolder)
    const fakeDestinyFolder = `${FOLDER_TO_TESTS}/DESTINY/`
    await promisify(fs.mkdir)(fakeDestinyFolder)
    const parameters = await mountParameters({
      fileContent: [
        `${fakeFolder} ${fakeDestinyFolder}`,
        `- 10 move ${fakeDestinyFolder}`
      ]
    })
    const newFakeFile = await createFakeFile(fakeFolder, 5)
    await sut.run(parameters.parsed)
    const filesOrigin = await promisify(fs.readdir)(fakeFolder)
    const filesDestiny = await promisify(fs.readdir)(fakeDestinyFolder)
    expect(filesOrigin).toEqual([ newFakeFile ])
    expect(filesDestiny).toEqual([])
  })

  test('deve mover arquivos que entraram no alcance de execução para a pasta destino informada', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/MOVE/`
    await promisify(fs.mkdir)(fakeFolder)
    const fakeDestinyFolder = `${FOLDER_TO_TESTS}/DESTINY/`
    await promisify(fs.mkdir)(fakeDestinyFolder)
    const parameters = await mountParameters({
      fileContent: [
        `${fakeFolder} ${fakeDestinyFolder}`,
        `- 10 move ${fakeDestinyFolder}`
      ]
    })
    const newFakeFile = await createFakeFile(fakeFolder, 5)
    const fakeFileToMove = await createFakeFile(fakeFolder, 20)
    await sut.run(parameters.parsed)
    const filesOrigin = await promisify(fs.readdir)(fakeFolder)
    const filesDestiny = await promisify(fs.readdir)(fakeDestinyFolder)
    expect(filesOrigin).toEqual([ newFakeFile ])
    expect(filesDestiny).toEqual([ fakeFileToMove ])
  })

  test('se informado a data de exclusão dos arquivos, estes devem ser removidos quando entrarem no alcance', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/MOVE/`
    await promisify(fs.mkdir)(fakeFolder)
    const fakeDestinyFolder = `${FOLDER_TO_TESTS}/DESTINY/`
    await promisify(fs.mkdir)(fakeDestinyFolder)
    const parameters = await mountParameters({
      fileContent: [
        `${fakeFolder} ${fakeDestinyFolder}`,
        `- 10 move ${fakeDestinyFolder} 20`
      ]
    })
    const newFakeFile = await createFakeFile(fakeFolder, 5)
    const fakeFileToMove = await createFakeFile(fakeFolder, 15)
    await createFakeFile(fakeDestinyFolder, 25)
    await sut.run(parameters.parsed)
    const filesOrigin = await promisify(fs.readdir)(fakeFolder)
    const filesDestiny = await promisify(fs.readdir)(fakeDestinyFolder)
    expect(filesOrigin).toEqual([ newFakeFile ])
    expect(filesDestiny).toEqual([ fakeFileToMove ])
  })
})

describe('\n  REMOVE files', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve ignorar arquivos recentes que ainda não estão no alcance de execução', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/REMOVE/`
    await promisify(fs.mkdir)(fakeFolder)
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, `- 10 remove` ]
    })
    const fakeFile = await createFakeFile(fakeFolder, 5)
    await sut.run(parameters.parsed)
    const files = await promisify(fs.readdir)(fakeFolder)
    expect(files).toContain(fakeFile)
  })

  test('deve remover arquivos que entraram no alcance de execução', async () => {
    const sut = makeSut()
    const fakeFolder = `${FOLDER_TO_TESTS}/REMOVE/`
    await promisify(fs.mkdir)(fakeFolder)
    const parameters = await mountParameters({
      fileContent: [ fakeFolder, `- 10 remove` ]
    })
    const newFakeFile = await createFakeFile(fakeFolder, 5)
    await createFakeFile(fakeFolder, 21)
    await sut.run(parameters.parsed)
    const files = await promisify(fs.readdir)(fakeFolder)
    expect(files).toEqual([ newFakeFile ])
  })
})
