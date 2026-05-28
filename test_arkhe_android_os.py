import pytest
import os
import json
from unittest.mock import patch
from arkhe_android_os import AndroidOSConfig, AndroidPackageStructure, KotlinSourceGenerator, GradleBuildGenerator, AOSPIntegration, ArkheAndroidOS

def test_config():
    config = AndroidOSConfig()
    assert config.target_sdk == 34
    assert "920" in config.substrate_modules
    assert "928" in config.substrate_modules
    assert "sensors" in config.hal_modules

def test_package_structure():
    assert AndroidPackageStructure.PACKAGE_ROOT == "cathedral.arkhe.os"
    config = AndroidOSConfig()
    manifest = AndroidPackageStructure.generate_manifest(config)
    assert 'package="cathedral.arkhe.os"' in manifest
    assert "android.permission.INTERNET" in manifest

def test_kotlin_generator():
    config = AndroidOSConfig()
    app = KotlinSourceGenerator.generate_arkhe_application(config)
    assert "class ArkheApplication : Application()" in app
    assert '"920"' in app

def test_build_generator():
    config = AndroidOSConfig()
    gradle = GradleBuildGenerator.generate_build_gradle_app(config)
    assert 'namespace = "cathedral.arkhe.os"' in gradle

def test_aosp_integration():
    mk = AOSPIntegration.generate_aosp_mk()
    assert "LOCAL_PACKAGE_NAME := ARKHEOS" in mk
    se = AOSPIntegration.generate_selinux_policy()
    assert "type arkhe_app, domain;" in se

def test_android_os():
    os_gen = ArkheAndroidOS()
    project = os_gen.generate_project()
    assert project["status"] == "generated"
    assert "manifest" in project["structure"]
    assert "kotlin_sources" in project["structure"]

if __name__ == "__main__":
    pytest.main([__file__, "-v"])
