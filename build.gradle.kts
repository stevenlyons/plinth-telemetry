// Root build file. Plugin versions are managed via gradle/libs.versions.toml.
// Each submodule applies plugins with `apply false` here so versions stay in one place.

plugins {
    alias(libs.plugins.android.application) apply false
    alias(libs.plugins.android.library) apply false
    alias(libs.plugins.kotlin.android) apply false
    alias(libs.plugins.kotlin.serialization) apply false
}
