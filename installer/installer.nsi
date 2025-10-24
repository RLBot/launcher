; NSI installer script for RLBot5
; -----
; See https://youtu.be/5HcLY8g5rSs?si=5oRHeqnTH_qyfEP0
; See https://nsis.sourceforge.io/Docs/Chapter4.html

!include "MUI.nsh"

Name "RLBot5"
InstallDir "$LOCALAPPDATA\RLBot5"
OutFile "rlbot5-installer.exe"
BrandingText "RLBot"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_INSTFILES
    !define MUI_FINISHPAGE_RUN
    !define MUI_FINISHPAGE_RUN_TEXT "Run RLBot5 Launcher"
    !define MUI_FINISHPAGE_RUN_FUNCTION "RunPostInstall"
!insertmacro MUI_PAGE_FINISH

; Unpages
!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

; Languages
!insertmacro MUI_LANGUAGE "English"

; -------------------
Section ""
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "DisplayName" "RLBot5"
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "DisplayVersion" "1.0.0.0"
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "Publisher" "RLBot"
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "DisplayIcon" "$INSTDIR\icon.ico"
    WriteRegDWORD HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "NoModify" 1
    WriteRegDWORD HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "NoRepair" 1
    WriteRegStr HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5" "UninstallString" "$INSTDIR\Uninstall.exe"

    SetOutPath $INSTDIR
    WriteUninstaller $INSTDIR\Uninstall.exe
    File "..\assets\icon.ico"

    SetOutPath $INSTDIR\bin
    File "..\target\release\launcher.exe"

    CreateShortcut "$SMPROGRAMS\RLBot5 Launcher.lnk" "$INSTDIR\bin\launcher.exe"
SectionEnd

Section "Uninstall"
    DeleteRegKey HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\RLBot5"

    RMDir /r $INSTDIR

    Delete "$SMPROGRAMS\RLBot5 Launcher.lnk"
SectionEnd

Function RunPostInstall
    ExecShell "" "$INSTDIR\bin\launcher.exe"
FunctionEnd