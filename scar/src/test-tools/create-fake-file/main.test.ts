import fs from 'fs'
import { promisify } from 'util'
import 'dotenv/config'
import { Bash } from "../../tools/Bash";
import { removeFolderToTests } from '../removeFolderToTests';
import { createFakeFolder } from '../createFakeFolder';
import { createFolderToTests } from '../createFolderToTests';

function makeSut() {
  const PROJECT_FOLDER = process.env.PROJECT_FOLDER
  if (PROJECT_FOLDER === undefined) {
    throw Error('Defina a variável PROJECT_FOLDER no arquivo .env.tests')
  }

  return new Bash(`${PROJECT_FOLDER}/src/test-tools/create-fake-file/main.sh`)
}

describe('test-tools/create-fake-file/main.sh', () => {
  beforeEach(async () => {
    await createFolderToTests()
  })

  afterEach(async () => {
    await removeFolderToTests()
  })

  test('deve encerrar com erro caso a pasta destino não seja informada "$1"', async () => {
    const sut = makeSut()
    const expectedError = 'deve ser informada a pasta em que o arquivo deve ser criado - "$1"'
    await expect(sut.run('')).rejects.toThrowError(expectedError)
  })

  test('deve encerrar com erro caso a pasta destino não exista "$1"', async () => {
    const sut = makeSut()
    const expectedError = 'não foi encontrada a pasta em que o arquivo deve ser criado - "$1"'
    await expect(sut.run('/any/folder')).rejects.toThrowError(expectedError)
  })

  test('deve encerrar com erro caso o ID do arquivo não seja informado "$2"', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const expectedError = 'deve ser informado o ID do arquivo a ser criado - "$2"'
    await expect(sut.run(fakeFolder)).rejects.toThrowError(expectedError)
  })

  test('deve encerrar com erro caso o tamanho do arquivo não seja informado "$3"', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const expectedError = 'o tamanho do arquivo deve ser informado (Mb) - "$3"'
    await expect(sut.run(`${fakeFolder} ID`)).rejects.toThrowError(expectedError)
  })

  test('deve encerrar com erro caso o tamanho do arquivo não seja um número inteiro "$3"', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const expectedError = 'o tamanho do arquivo deve ser um número inteiro (Mb) - "$3"'
    await expect(sut.run(`${fakeFolder} ID any`)).rejects.toThrowError(expectedError)
  })

  test('deve encerrar com erro caso seja informado um dia inválido para envelhecer o arquivo "$4"', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const expectedError = 'o dia de criação do arquivo deve ser um número inteiro - "$4"'
    await expect(sut.run(`${fakeFolder} ID 1 any`)).rejects.toThrowError(expectedError)
  })

  test('caso todos os parâmetros sejam informados corretamente, não deve lançar erro', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await expect(sut.run(`${fakeFolder} ID 1`)).resolves.not.toThrow()
  })

  test('deve criar um arquivo com o ID informado no local indicado', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    await sut.run(`${fakeFolder} ID 1`)
    const files = await promisify(fs.readdir)(fakeFolder)
    expect(files).toContain('file-ID')
  })

  test('deve criar um arquivo com o tamanho informado no local indicado', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const randomSize = Math.round(Math.random() * 10)
    await sut.run(`${fakeFolder} ID ${randomSize}`)
    const fileStatus = await promisify(fs.stat)(`${fakeFolder}/file-ID`)
    expect(fileStatus.size / (1024*1024)).toBe(randomSize)
  })

  test('deve criar um arquivo com data de "dias atrás" que for informado informado', async () => {
    const sut = makeSut()
    const fakeFolder = await createFakeFolder()
    const randomDaysAgo = Math.round(Math.random() * 400)
    await sut.run(`${fakeFolder} ID 1 ${randomDaysAgo}`)
    const fileStatus = await promisify(fs.stat)(`${fakeFolder}/file-ID`)
    const correctDate = new Date()
    correctDate.setDate(correctDate.getDate() - randomDaysAgo)
    expect({
      year: fileStatus.atime.getFullYear(),
      month: fileStatus.atime.getMonth(),
      day: fileStatus.atime.getDate()
    }).toEqual({
      year: correctDate.getFullYear(),
      month: correctDate.getMonth(),
      day: correctDate.getDate()
    })
  })
})
