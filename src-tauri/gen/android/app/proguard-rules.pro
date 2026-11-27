# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.
#
# For more details, see
#   http://developer.android.com/guide/developing/tools/proguard.html

# If your project uses WebView with JS, uncomment the following
# and specify the fully qualified class name to the JavaScript interface
# class:
#-keepclassmembers class fqcn.of.javascript.interface.for.webview {
#   public *;
#}

# Uncomment this to preserve the line number information for
# debugging stack traces.
#-keepattributes SourceFile,LineNumberTable

# If you keep the line number information, uncomment this to
# hide the original source file name.
#-renamesourcefileattribute SourceFile

# Shizuku creates StorageService by reflection in a separate shell process.
# R8 cannot see that construction, so without these rules it strips the
# no-arg constructor and renames the binder surface in release builds.
-keep class net.solarengine.solarlanddroid.StorageService { *; }
-keep class net.solarengine.solarlanddroid.IStorageService { *; }
-keep class net.solarengine.solarlanddroid.IStorageService$Stub { *; }
-keep class net.solarengine.solarlanddroid.IStorageService$Stub$Proxy { *; }
-keep class net.solarengine.solarlanddroid.IStorageService$Default { *; }