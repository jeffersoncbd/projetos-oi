import { exec } from 'child_process'

export class AcceptableError extends Error {
  constructor(feedback: string) {
    super(feedback)
    Object.setPrototypeOf(this, AcceptableError.prototype)
    this.name = 'AcceptableError'
  }
}

export class Bash {
  constructor(private scriptPath: string) {}

  run(parameters: string): Promise<string> {
    return new Promise((resolve, rejects) => {
      exec(`${this.scriptPath} ${parameters}`, (error, stdout, stderr) => {
        if (error) {
          if (error.code === 4) {
            rejects(new AcceptableError(stderr))
          } else {
            rejects(new Error(stderr))
          }
        }
        resolve(stdout)
      })
    })
  }
}
