import fs from "fs/promises"
import path from "path"

export const readLatestEmail = async () => {
  try {
    const files = await fs.readdir("/tmp")
    const emailFiles = files.filter((file) => file.endsWith(".eml"))

    if (emailFiles.length === 0) return null

    const fileStats = await Promise.all(
      emailFiles.map(async (file) => {
        const stat = await fs.stat(path.join("/tmp", file))
        return {
          name: file,
          ctime: stat.ctime,
        }
      })
    )

    const latestFile = fileStats.sort((a, b) => b.ctime.getTime() - a.ctime.getTime())[0]?.name

    if (!latestFile) return null

    const content = await fs.readFile(path.join("/tmp", latestFile), "utf-8")

    // Delete file
    await fs.unlink(path.join("/tmp", latestFile))

    return content
  } catch (err) {
    console.error(err)
    return null
  }
}

export const extractContinueToken = (content: string): string | null | undefined => {
  // Remove quoted-printable line continuations (= at end of line)
  const cleanedContent = content.replace(/=\r?\n/g, "")
  // Match the token after continue/
  const tokenMatch = cleanedContent.match(/continue\/([a-zA-Z0-9]+)/)
  return tokenMatch ? tokenMatch[1] : null
}
