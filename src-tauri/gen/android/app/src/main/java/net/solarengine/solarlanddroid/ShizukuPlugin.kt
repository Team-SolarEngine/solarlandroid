package net.solarengine.solarlanddroid

import android.app.Activity
import android.content.ComponentName
import android.content.ServiceConnection
import android.content.pm.PackageManager
import android.os.IBinder
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import org.json.JSONArray
import rikka.shizuku.Shizuku

/**
 * Tauri bridge to a Shizuku user service that can read and write
 * /sdcard/Android/obb/... with shell privileges.
 */
@TauriPlugin
class ShizukuPlugin(private val activity: Activity) : Plugin(activity) {
  private var service: IStorageService? = null
  private var bound = false
  private var pendingPermission: Invoke? = null

  private val connection = object : ServiceConnection {
    override fun onServiceConnected(name: ComponentName?, binder: IBinder?) {
      service = binder?.takeIf { it.pingBinder() }?.let { IStorageService.Stub.asInterface(it) }
    }

    override fun onServiceDisconnected(name: ComponentName?) {
      service = null
      bound = false
    }
  }

  init {
    Shizuku.addBinderReceivedListenerSticky { bind() }

    Shizuku.addBinderDeadListener {
      service = null
      bound = false
    }

    Shizuku.addRequestPermissionResultListener { _, result ->
      if (result == PackageManager.PERMISSION_GRANTED) bind()
      pendingPermission?.resolve(
        status().put("permissionGranted", result == PackageManager.PERMISSION_GRANTED)
      )
      pendingPermission = null
    }
  }

  @Command
  fun status(invoke: Invoke) {
    invoke.resolve(status())
  }

  @Command
  fun requestPermission(invoke: Invoke) {
    try {
      if (!Shizuku.pingBinder()) {
        invoke.reject("Shizuku is not running. Install and start Shizuku first.")
        return
      }

      if (Shizuku.checkSelfPermission() == PackageManager.PERMISSION_GRANTED) {
        bind()
        invoke.resolve(status())
        return
      }

      pendingPermission = invoke
      Shizuku.requestPermission(PERMISSION_REQUEST_CODE)
    } catch (e: Throwable) {
      pendingPermission = null
      invoke.reject(e.message ?: "Could not request Shizuku permission")
    }
  }

  @Command
  fun listFolder(invoke: Invoke) {
    val path = invoke.getArgs().getString("path")
    val storage = requireService(invoke) ?: return

    try {
      invoke.resolve(JSObject().put("items", JSONArray(storage.list(path))))
    } catch (e: Throwable) {
      invoke.reject(e.message ?: "Could not list $path")
    }
  }

  @Command
  fun makeDirs(invoke: Invoke) {
    val path = invoke.getArgs().getString("path")
    runOperation(invoke) { it.makeDirs(path) }
  }

  @Command
  fun rename(invoke: Invoke) {
    val from = invoke.getArgs().getString("from")
    val to = invoke.getArgs().getString("to")
    runOperation(invoke) { it.rename(from, to) }
  }

  @Command
  fun delete(invoke: Invoke) {
    val path = invoke.getArgs().getString("path")
    runOperation(invoke) { it.delete(path) }
  }

  @Command
  fun copy(invoke: Invoke) {
    val from = invoke.getArgs().getString("from")
    val to = invoke.getArgs().getString("to")
    runOperation(invoke) { it.copy(from, to) }
  }

  @Command
  fun openFolder(invoke: Invoke) {
    val path = invoke.getArgs().getString("path")
    runOperation(invoke) { it.openFolder(path) }
  }

  private fun runOperation(invoke: Invoke, operation: (IStorageService) -> String?) {
    val storage = requireService(invoke) ?: return
    try {
      val error = operation(storage)
      if (error.isNullOrEmpty()) {
        invoke.resolve(JSObject().put("ok", true))
      } else {
        invoke.reject(error)
      }
    } catch (e: Throwable) {
      invoke.reject(e.message ?: "Storage operation failed")
    }
  }

  private fun requireService(invoke: Invoke): IStorageService? {
    val current = service
    if (current != null && current.asBinder().pingBinder()) return current

    bind()
    invoke.reject("Shizuku storage service is not ready. Start Shizuku, grant access, then retry.")
    return null
  }

  private fun bind() {
    if (bound) return

    try {
      if (Shizuku.isPreV11() || !Shizuku.pingBinder() || Shizuku.getVersion() < 10) return
      if (Shizuku.checkSelfPermission() != PackageManager.PERMISSION_GRANTED) return

      val args = Shizuku.UserServiceArgs(
        ComponentName(activity.packageName, StorageService::class.java.name)
      )
        .daemon(false)
        .processNameSuffix("storage")
        .version(1)

      Shizuku.bindUserService(args, connection)
      bound = true
    } catch (_: Throwable) {
      bound = false
    }
  }

  private fun status(): JSObject {
    val result = JSObject()
      .put("binder", false)
      .put("permissionGranted", false)
      .put("serviceReady", service != null)

    try {
      result.put("binder", Shizuku.pingBinder())
      if (!Shizuku.isPreV11() && Shizuku.pingBinder()) {
        result
          .put("permissionGranted", Shizuku.checkSelfPermission() == PackageManager.PERMISSION_GRANTED)
          .put("uid", Shizuku.getUid())
      }
    } catch (e: Throwable) {
      result.put("error", e.message ?: "Shizuku status failed")
    }

    return result
  }

  companion object {
    private const val PERMISSION_REQUEST_CODE = 1001
  }
}
