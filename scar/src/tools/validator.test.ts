import fs from 'fs'
import { promisify } from 'util'
import 'dotenv/config'

import { Bash } from "./Bash"
import { createFakeFolder, removeFolderToTests } from '../test-tools'
import { createFolderToTests } from '../test-tools/createFolderToTests'


function makeSut() {
  const PROJECT_FOLDER = process.env.PROJECT_FOLDER
  if (PROJECT_FOLDER === undefined) {
    throw Error('Defina a variável PROJECT_FOLDER no arquivo .env.tests')
  }

  return new Bash(`${PROJECT_FOLDER}/src/tools/validator.sh`)
}

interface Parameters {
  errorMessage?: string
  fileContent: string[]
  rowsWithErrors?: number[]
}

export async function mountParameters(parameters: Parameters) {
  const FOLDER_TO_TESTS = process.env.FOLDER_TO_TESTS
  if (FOLDER_TO_TESTS === undefined) {
    throw Error('Defina a variável FOLDER_TO_TESTS no arquivo .env.tests')
  }

  const { errorMessage, fileContent, rowsWithErrors } = parameters

  const filePath = `${FOLDER_TO_TESTS}/fake-file`
  await promisify(fs.writeFile)(filePath, fileContent.join('\n'))

  const parsed = `${filePath} ${FOLDER_TO_TESTS}`

  const title = errorMessage ? errorMessage.replace('{parameters.filePath}', filePath) : ''
  const rows = rowsWithErrors?.map((number) => `${filePath}: linha ${number}: ${fileContent[number - 1] || ''}`)
  const expectedError =
    `${title}${rows ? `\n${rows.join('\n')}` : ''}`

  return { filePath, destinyPath: FOLDER_TO_TESTS as string, parsed, expectedError }
}

describe('folder list validator', () => {
  beforeEach(async () => {
    await createFolderToTests()
  })

  afterEach(async () => {
    await removeFolderToTests()
  })

  test('deve finalizar o script com erro se o parâmetro "$1" não for informado.', async () => {
    const sut = makeSut()
    const expectedError = 'O parâmetro "$1" (lista de pastas do usuário) não foi informado.'
    await expect(sut.run('')).rejects.toThrowError(expectedError)
  })

  test('deve finalizar o script com erro se o parâmetro "$2" não for informado', async () => {
    const sut = makeSut()
    const expectedError = 'O parâmetro "$2" (destino do arquivo validado) não foi informado.'
    await expect(sut.run(`/any/file`)).rejects.toThrowError(expectedError)
  })

  test('deve finalizar o script com erro se o arquivo informado não existir ("$1").', async () => {
    const sut = makeSut()
    const fakeFile = '/any/file'
    const parameters = `${fakeFile} /any/validated-file-folder/`
    const expectedError = `O arquivo "${fakeFile}" não existe.`
    await expect(sut.run(parameters)).rejects.toThrowError(expectedError)
  })

  test('deve finalizar o script com erro se o caminho para a pasta destino do arquivo validado não existir', async () => {
    const sut = makeSut()
    const fakeFolder = '/any/validated-file-folder/'
    const parameters = await mountParameters({
      errorMessage: `A pasta para salvar o arquivo validado "${fakeFolder}" não existe.`,
      fileContent: ['']
    })
    await expect(sut.run(`${parameters.filePath} ${fakeFolder}`))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se o arquivo informado estiver vazio.', async () => {
    const sut = makeSut()
    const parameters = await mountParameters({
      errorMessage: `O arquivo "{parameters.filePath}" está vazio.`,
      fileContent: ['']
    })
    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se o arquivo informado só contiver comentários.', async () => {
    const sut = makeSut()
    const parameters = await mountParameters({
      errorMessage: `O arquivo "{parameters.filePath}" está vazio.`,
      fileContent: ['# any commentary']
    })
    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se existir linhas com flag não aceita.', async () => {
    const sut = makeSut()
    const parameters = await mountParameters({
      errorMessage: `As linhas devem começar com "/" ou "-"`,
      fileContent: ['# row 1', '# row 2', '# row 3', '* invalid row'],
      rowsWithErrors: [4]
    })
    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se existir pasta sem comando.' , async () => {
    const sut = makeSut()
    const parameters = await mountParameters({
      errorMessage: 'Toda pasta deve ter ao menos um comando (iniciado por "-")',
      fileContent: ['# row 1', '# row 2', '/any/folder'],
      rowsWithErrors: [3,4]
    })
    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se pasta informada não existir', async () => {
    const sut = makeSut()
    const parameters = await mountParameters({
      errorMessage: 'Uma pasta informada no arquivo não foi encontrada',
      fileContent: ['# row1', '/any/folder', '- any command'],
      rowsWithErrors: [2]
    })
    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se algum comando estiver vazio', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const parameters = await mountParameters({
      errorMessage: 'Uma pasta possui linha de comandos vazia',
      fileContent: ['# row1', fakeFolder, '-'],
      rowsWithErrors: [3]
    })
    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se algum comando não informar [d1]', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()

    const parameters = await mountParameters({
      errorMessage: 'Um comando não possui "d1" especificado (dias para execução)',
      fileContent: ['# row1', fakeFolder, '- error'],
      rowsWithErrors: [3]
    })

    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se algum comando não for aceito', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const parameters = await mountParameters({
      errorMessage: 'O comando "invalid" não é aceito (use "zip", "move" ou "remove")',
      fileContent: ['# row 1', fakeFolder, '- 10 invalid'],
      rowsWithErrors: [3]
    })
    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se algum comando informar [d2] inválido', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const parameters = await mountParameters({
      errorMessage: 'Um comando possui um "d2" inválido (dias para remoção) utilize apenas números inteiros',
      fileContent: ['# row 1', fakeFolder, '- 10 remove invalid'],
      rowsWithErrors: [3]
    })
    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se o comando for "move" e o destino não for informado', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const parameters = await mountParameters({
      errorMessage: 'Quando utilizado o comando "move" deve ser informado o caminho de destino',
      fileContent: ['# row 1', fakeFolder, '- 30 move'],
      rowsWithErrors: [3]
    })
    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve finalizar o script com erro se o comando for "move" e o destino não for válido', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const parameters = await mountParameters({
      errorMessage: 'O caminho do destino para o comando "move" é inválido',
      fileContent: [ fakeFolder, '- 10 move /any/folder' ],
      rowsWithErrors: [2]
    })
    await expect(sut.run(parameters.parsed))
      .rejects.toThrowError(parameters.expectedError)
  })

  test('deve fazer uma cópia do arquivo válido para a pasta informada', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const parameters = await mountParameters({
      fileContent: [fakeFolder, '- 10 remove 10']
    })
    await sut.run(parameters.parsed)
    const files = await promisify(fs.readdir)(parameters.destinyPath)
    expect(files).toContain('valid-list-of-folders')
  })

  test('a copia do arquivo deve ter o mesmo conteúdo que o original', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const fileContent = [fakeFolder, '- 10 remove 10']
    const parameters = await mountParameters({ fileContent })
    await sut.run(parameters.parsed)
    const copyContent =
      (await promisify(fs.readFile)(`${parameters.destinyPath}/valid-list-of-folders`)).toString()
    expect(fileContent.join('\n')).toEqual(copyContent)
  })

  test('não deve validar [d2] se o mesmo for omitido', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const fileContent = [fakeFolder, '- 10 remove']
    const parameters = await mountParameters({ fileContent })
    await expect(sut.run(parameters.parsed)).resolves.not.toThrow()
  })

  test('espaços adicionais em qualquer lugar do arquivo não deve gerar erro', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const fileContent = [
      '                       ',
      '         #  comment             ',
      `                ${fakeFolder}   `,
      '     -       10     zip  30 ',
      '     #  any comment '
    ]
    const parameters = await mountParameters({ fileContent })
    await expect(sut.run(parameters.parsed)).resolves.not.toThrow()
  })
})
