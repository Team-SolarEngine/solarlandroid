package net.solarengine.solarlanddroid

import android.content.Intent
import android.net.Uri
import org.json.JSONArray
import org.json.JSONObject
import java.io.File

/**
 * Runs as the Shizuku user service, so these file operations happen with
 * shell/root identity and can reach /sdcard/Android/obb.
 */
class StorageService : IStorageService.Stub() {
  override fun destroy() {
    System.exit(0)
  }

  override fun list(path: String): String {
    val array = JSONArray()
    val dir = File(path)
    if (!dir.isDirectory) return array.toString()

    for (file in dir.listFiles() ?: emptyArray()) {
      array.put(
        JSONObject()
          .put("path", file.absolutePath)
          .put("isFolder", file.isDirectory)
      )
    }
    return array.toString()
  }

  override fun makeDirs(path: String): String = runCatching {
    if (!File(path).isDirectory && !File(path).mkdirs()) {
      "Failed to create $path"
    } else {
      ""
    }
  }.getOrElse { it.message ?: "Failed to create $path" }

  override fun rename(from: String, to: String): String = runCatching {
    val source = File(from)
    val target = File(to)
    target.parentFile?.mkdirs()

    if (!source.renameTo(target)) {
      "Failed to move $from to $to"
    } else {
      ""
    }
  }.getOrElse { it.message ?: "Failed to move $from" }

  override fun delete(path: String): String = runCatching {
    val target = File(path)
    if (target.exists() && !target.deleteRecursively()) {
      "Failed to delete $path"
    } else {
      ""
    }
  }.getOrElse { it.message ?: "Failed to delete $path" }

  override fun copy(from: String, to: String): String = runCatching {
    val source = File(from)
    val target = File(to)
    target.parentFile?.mkdirs()
    source.copyRecursively(target, overwrite = true)
    ""
  }.getOrElse { it.message ?: "Failed to copy $from" }

  override fun openFolder(path: String): String = runCatching {
    val normalized = path.replace('\\', '/')
    val relative = when {
      normalized.startsWith("/storage/emulated/0/") ->
        normalized.removePrefix("/storage/emulated/0/")
      normalized.startsWith("/sdcard/") ->
        normalized.removePrefix("/sdcard/")
      else -> normalized.trimStart('/')
    }
    val uri = "content://com.android.externalstorage.documents/document/" +
      Uri.encode("primary:$relative")

    val process = ProcessBuilder(
      "am", "start", "-a", Intent.ACTION_VIEW, "-d", uri
    ).redirectErrorStream(true).start()

    val output = process.inputStream.bufferedReader().use { it.readText() }.trim()
    val exit = process.waitFor()

    if (exit != 0 || output.contains("Error", ignoreCase = true)) {
      output.ifBlank { "Failed to open $path" }
    } else {
      ""
    }
  }.getOrElse { it.message ?: "Failed to open $path" }
}
