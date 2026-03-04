Var strCustomComputerName

${SegmentFile}

${SegmentInit}
	${registry::Read} "HKLM\SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName" "ComputerName" $strCustomComputerName $0

	${If} $Bits == 32
		Rename "$EXEDIR\Data\HWiNFO64.ini" "$EXEDIR\Data\HWiNFO32.ini"
	${Else}
		Rename "$EXEDIR\Data\HWiNFO32.ini" "$EXEDIR\Data\HWiNFO64.ini"
	${EndIf}
!macroend

${SegmentPreExecPrimary}
	${If} ${FileExists} "$EXEDIR\Data\settings\HWiNFO32-$strCustomComputerName.reg"
		${If} ${FileExists} "$EXEDIR\Data\settings\HWiNFO32.reg"
			Rename "$EXEDIR\Data\settings\HWiNFO32.reg" "$EXEDIR\Data\settings\HWiNFO32-Unknown.reg"
		${EndIf}
		Rename "$EXEDIR\Data\settings\HWiNFO32-$strCustomComputerName.reg" "$EXEDIR\Data\settings\HWiNFO32.reg"
	${EndIf}
	${If} ${FileExists} "$EXEDIR\Data\settings\HWiNFO64-$strCustomComputerName.reg"
		${If} ${FileExists} "$EXEDIR\Data\settings\HWiNFO64.reg"
			Rename "$EXEDIR\Data\settings\HWiNFO64.reg" "$EXEDIR\Data\settings\HWiNFO64-Unknown.reg"
		${EndIf}
		Rename "$EXEDIR\Data\settings\HWiNFO64-$strCustomComputerName.reg" "$EXEDIR\Data\settings\HWiNFO64.reg"
	${EndIf}
	${If} $Bits == 64
		CreateDirectory "$PROGRAMFILES64\HWiNFOPortableTemp"
		CopyFiles /SILENT "$EXEDIR\App\HWiNFO\HWiNFO6*.*" "$PROGRAMFILES64\HWiNFOPortableTemp"
	${EndIf}
!macroend


${SegmentPostPrimary}
	Delete "$EXEDIR\Data\settings\HWiNFO32-$strCustomComputerName.reg"
	Rename "$EXEDIR\Data\settings\HWiNFO32.reg" "$EXEDIR\Data\settings\HWiNFO32-$strCustomComputerName.reg"
	Delete "$EXEDIR\Data\settings\HWiNFO64-$strCustomComputerName.reg"
	Rename "$EXEDIR\Data\settings\HWiNFO64.reg" "$EXEDIR\Data\settings\HWiNFO64-$strCustomComputerName.reg"
	${If} $Bits == 64
		Rename "$EXEDIR\Data\HWiNFO64.ini" "$EXEDIR\Data\HWiNFO64-backup.ini"
		Rename "$EXEDIR\Data\HWiNFO64_KEY.txt" "$EXEDIR\Data\HWiNFO64_KEY-backup.txt"
		CopyFiles /SILENT "$PROGRAMFILES64\HWiNFOPortableTemp\HWiNFO64.ini" "$EXEDIR\Data"
		CopyFiles /SILENT "$PROGRAMFILES64\HWiNFOPortableTemp\HWiNFO64_KEY.txt" "$EXEDIR\Data"
		RMDir /r "$PROGRAMFILES64\HWiNFOPortableTemp"
		${If} ${FileExists} "$EXEDIR\Data\HWiNFO64.ini"
			Delete "$EXEDIR\Data\HWiNFO64-backup.ini"
		${EndIf}
		${If} ${FileExists} "$EXEDIR\Data\HWiNFO64_KEY.txt"
			Delete "$EXEDIR\Data\HWiNFO64_KEY-backup.txt"
		${EndIf}
	${EndIf}
!macroend
