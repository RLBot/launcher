@echo off

echo Running candle
candle installer\\installer.wxs -ext WixUIExtension -ext WixUtilExtension -out ./installer/installer.wixobj -nologo

echo Running light
light installer\\installer.wixobj -ext WixUIExtension -ext WixUtilExtension -spdb -nologo -out ./installer/rlbot-v5-installer.msi -sval

echo Cleaning up
del installer\\installer.wix*
