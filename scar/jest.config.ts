export default {
  roots: ['<rootDir>/src'],
  clearMocks: true,
  collectCoverage: false,
  testEnvironment: 'node',
  moduleFileExtensions: ["js", "ts", "sh"],
  testMatch: ['<rootDir>/src/**/*.test.ts'],
  preset: 'ts-jest'
}
