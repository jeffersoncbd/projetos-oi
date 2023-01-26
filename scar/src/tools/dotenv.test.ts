import { promisify } from 'util'
import fs from 'fs'
import { createFakeFolder, removeFolderToTests } from "../test-tools";
import { createFolderToTests } from "../test-tools/createFolderToTests";
import { Bash } from "./Bash";

const PROJECT_FOLDER = process.env.PROJECT_FOLDER
if (PROJECT_FOLDER === undefined) {
  throw Error('Defina a variável PROJECT_FOLDER no arquivo .env.tests')
}

function makeSut() {
  const PROJECT_FOLDER = process.env.PROJECT_FOLDER
  if (PROJECT_FOLDER === undefined) {
    throw Error('Defina a variável PROJECT_FOLDER no arquivo .env.tests')
  }
  return new Bash(`${PROJECT_FOLDER}/src/tools/dotenv.sh`)
}

describe('start.sh', () => {
  beforeEach(async () => await createFolderToTests())
  afterEach(async () => await removeFolderToTests())

  test('deve encerrar o script com erro se o caminho para a pasta do projeto não for informado', async () => {
    const sut = makeSut()
    const expectedError = 'o caminho para a pasta do projeto não foi informado \"\$1\"'
    await expect(sut.run('')).rejects.toThrowError(expectedError)
  })

  test('deve buscar um arquivo .env.dev se o o tipo de ambiente for informado como "dev"', async () => {
    const sut = makeSut()
    const fakePath = '/any/folder'
    const expectedError = `não foi encontrado um arquivo ".env.dev" na pasta "${fakePath}/", crie um usando o .env.example como base`
    await expect(sut.run(`${fakePath} dev`)).rejects.toThrowError(expectedError)
  })

  test('deve encerrar o script com erro se não for encontrado o arquivo .env', async () => {
    const sut = makeSut()
    const fakePath = '/any/folder'
    const expectedError = `não foi encontrado um arquivo ".env" na pasta "${fakePath}/", crie um usando o .env.example como base`
    await expect(sut.run(fakePath)).rejects.toThrowError(expectedError)
  })

  test('deve encerrar sem nenhum erro caso tudo esteja correto', async () => {
    const sut = makeSut()
    const fakePath = await createFakeFolder()
    await promisify(fs.writeFile)(`${fakePath}/.env`, '')
    await expect(sut.run(fakePath)).resolves.not.toThrow()
  })

  test('deve imprimir o array de variáveis caso seja informado 1 no segundo parâmetro', async () => {
    const sut = makeSut()
    const fakePath = await createFakeFolder()
    const fileContent = 'KEY="any_value"'
    await promisify(fs.writeFile)(`${fakePath}/.env`, fileContent)
    const result = await sut.run(`${fakePath} any dotenv`)
    expect(result).toBe(`${fileContent}\n`)
  })

  test('deve ignorar comentários de fim de linha', async () => {
    const sut = makeSut()
    const fakePath = await createFakeFolder()
    const fileContent = 'KEY="ANY_VALUE"'
    await promisify(fs.writeFile)(`${fakePath}/.env`, `${fileContent} # ANY COMMENTARY`)
    const result = await sut.run(`${fakePath} any dotenv`)
    expect(result).toBe(`${fileContent}\n`)
  })
})
