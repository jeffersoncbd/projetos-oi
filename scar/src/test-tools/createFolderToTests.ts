import { promisify } from 'util'
import fs from 'fs'
import 'dotenv/config'

export async function createFolderToTests() {
  const FOLDER_TO_TESTS = process.env.FOLDER_TO_TESTS
  if (FOLDER_TO_TESTS === undefined) {
    throw Error('Defina a variável FOLDER_TO_TESTS no arquivo .env.tests')
  }

  const exists = await promisify(fs.exists)(FOLDER_TO_TESTS)
  if (!exists) {
    await promisify(fs.mkdir)(FOLDER_TO_TESTS)
  }
}
