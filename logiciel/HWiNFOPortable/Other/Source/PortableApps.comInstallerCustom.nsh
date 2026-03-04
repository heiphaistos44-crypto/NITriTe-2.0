!macro CustomCodePostInstall
	${If} ${FileExists} "$EXEDIR\HWiNFO64_KEY.txt"
	${AndIfNot} ${FileExists} "$INSTDIR\Data\HWiNFO64_KEY.txt"
		CreateDirectory "$INSTDIR\Data"
		CopyFiles /SILENT "$EXEDIR\HWiNFO64_KEY.txt" "$INSTDIR\Data"
		${IfNot} ${FileExists} "$INSTDIR\Data\*.ini"
			CopyFiles /SILENT "$INSTDIR\App\DefaultData\*.*" "$INSTDIR\Data"
		${EndIf}
	${EndIf}
!macroend