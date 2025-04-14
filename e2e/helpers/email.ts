import fs from "fs/promises"
import path from "path"

/**
 * Read the latest email from the temporary directory.
 * Uses GitHub Actions runner.temp if available, falls back to /tmp for local development
 */
export const readLatestEmail = async () => {
  try {
    const tempDir = process.env.RUNNER_TEMP || "/tmp"

    const files = await fs.readdir(tempDir)
    const emailFiles = files.filter((file) => file.endsWith(".eml"))

    if (emailFiles.length === 0) return null

    const fileStats = await Promise.all(
      emailFiles.map(async (file) => {
        const stat = await fs.stat(path.join(tempDir, file))
        return {
          name: file,
          ctime: stat.ctime,
        }
      })
    )

    const latestFile = fileStats.sort((a, b) => b.ctime.getTime() - a.ctime.getTime())[0]?.name

    if (!latestFile) return null

    const content = await fs.readFile(path.join(tempDir, latestFile), "utf-8")

    // Delete file
    await fs.unlink(path.join(tempDir, latestFile))

    return content
  } catch (err) {
    console.error(err)
    return null
  }
}

export const extractContinueToken = (content: string): string | null | undefined => {
  // Due to quoted-printable encoding a '=' is used at the end of line,
  // so remove it.
  const cleanedContent = content.replace(/=\r?\n/g, "")
  // Match the token after continue/
  const tokenMatch = cleanedContent.match(/continue\/([a-zA-Z0-9]+)/)
  return tokenMatch ? tokenMatch[1] : null
}
