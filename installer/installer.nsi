!include "MUI.nsh"

Name "RLBot5"
InstallDir "$LOCALAPPDATA\RLBot5"
OutFile "rlbot5-installer.exe"
BrandingText "RLBot"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

Section ""
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Run" "RLBot" "$INSTDIR\bin\launcher.exe"
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "DisplayName" "RLBot5"
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "DisplayVersion" "1.0.0.0"
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "Publisher" "RLBot"
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "DisplayIcon" "$INSTDIR\logo.ico"
    WriteRegDWORD HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "NoModify" 1
    WriteRegDWORD HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "NoRepair" 1
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "UninstallString" "$INSTDIR\uninstall.exe"

    SetOutPath $INSTDIR
    WriteUninstaller $INSTDIR\uninstall.exe
    File "..\assets\logo.ico"

    SetOutPath $INSTDIR\bin
    File "..\target\release\launcher.exe"

    CreateShortcut "$SMPROGRAMS\RLBot5 Launcher.lnk" "$INSTDIR\bin\launcher.exe"
SectionEnd

Section "Uninstall"
    DeleteRegKey HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5"
    DeleteRegKey HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Run"

    RMDir /r $INSTDIR

    Delete "$SMPROGRAMS\RLBot5 Launcher.lnk"
SectionEnd