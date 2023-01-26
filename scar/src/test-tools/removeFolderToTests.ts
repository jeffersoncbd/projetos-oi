import { promisify } from 'util'
import fs from 'fs'
import 'dotenv/config'

export async function removeFolderToTests(path?: string) {
  if (path === undefined) {
    const FOLDER_TO_TESTS = process.env.FOLDER_TO_TESTS
    if (FOLDER_TO_TESTS === undefined) {
      throw Error('Defina a variável FOLDER_TO_TESTS no arquivo .env.tests')
    }
    path = FOLDER_TO_TESTS
  }

  const files = await promisify(fs.readdir)(path)
  for (const file of files) {
    try {
      await promisify(fs.rm)(`${path}/${file}`)
    } catch (error: any) {
      if (error.code === 'ERR_FS_EISDIR') {
        await removeFolderToTests(`${path}/${file}`)
      }
    }
  }
  await promisify(fs.rmdir)(path)
}
