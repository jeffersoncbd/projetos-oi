import fs from 'fs'
import { promisify } from 'util'
import 'dotenv/config'

const FOLDER_TO_TESTS = process.env.FOLDER_TO_TESTS
if (FOLDER_TO_TESTS === undefined) {
  throw Error('Defina a variável FOLDER_TO_TESTS no arquivo .env.tests')
}

export async function createFakeFolder() {
  const now = (new Date()).getTime()
  const random = Math.round(Math.random() * 1000000)
  const name = `${now}-${random}`
  const path = `${FOLDER_TO_TESTS}/${name}/`
  await promisify(fs.mkdir)(path)
  return path
}
