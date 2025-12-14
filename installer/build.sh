# TODO: Make windows build.bat file too.
#       WiX runs natively on windows so it shouldn't be a problem

echo Running candle
docker run --rm -v $(pwd):/wix dactiv/wix candle ./installer/installer.wxs \
    -out ./installer/installer.wixobj -nologo

echo Running light
docker run --rm -v $(pwd):/wix dactiv/wix light ./installer/installer.wixobj \
    -ext WixUIExtension -ext WixUtilExtension \
    -spdb -nologo -out ./installer/rlbot-v5-installer.msi \
    -sval # See https://github.com/dactivllc/docker-wix?tab=readme-ov-file#known-issues

echo Cleaning up
rm ./installer/installer.wix*
