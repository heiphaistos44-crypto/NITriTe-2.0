;Copyright 2004-2024 John T. Haller of PortableApps.com

;Website: http://portableapps.com/PuTTYPortable

;This software is OSI Certified Open Source Software.
;OSI Certified is a certification mark of the Open Source Initiative.

;This program is free software; you can redistribute it and/or
;modify it under the terms of the GNU General Public License
;as published by the Free Software Foundation; either version 2
;of the License, or (at your option) any later version.

;This program is distributed in the hope that it will be useful,
;but WITHOUT ANY WARRANTY; without even the implied warranty of
;MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;GNU General Public License for more details.

;You should have received a copy of the GNU General Public License
;along with this program; if not, write to the Free Software
;Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

!define NAME "HWiNFOPortable64Shim"
!define FULLNAME "HWiNFO Portable 64 Shim"
!define APP "HWiNFO"
!define VER "1.0.0.0"
!define WEBSITE "https://portableapps.com/apps/utilities/hwinfo-portable"

;=== Program Details
Name "${FULLNAME}"
OutFile "..\..\App\HWiNFO\${NAME}.exe"
Caption "${FULLNAME} | PortableApps.com"
VIProductVersion "${VER}"
VIAddVersionKey ProductName "${FULLNAME}"
VIAddVersionKey Comments "Allows ${APP} to be run from a removable drive.  For additional details, visit ${WEBSITE}"
VIAddVersionKey CompanyName "PortableApps.com"
VIAddVersionKey LegalCopyright "John T. Haller"
VIAddVersionKey FileDescription "${FULLNAME}"
VIAddVersionKey FileVersion "${VER}"
VIAddVersionKey ProductVersion "${VER}"
VIAddVersionKey InternalName "${FULLNAME}"
VIAddVersionKey LegalTrademarks "PortableApps.com is a trademark of Rare Ideas, LLC."
VIAddVersionKey OriginalFilename "${NAME}.exe"
;VIAddVersionKey PrivateBuild ""
;VIAddVersionKey SpecialBuild ""

;=== Runtime Switches
CRCCheck On
WindowIcon Off
SilentInstall Silent
AutoCloseWindow True
RequestExecutionLevel admin
XPStyle On
Unicode true
ManifestDPIAware true

;=== Program Icon
Icon "..\..\App\AppInfo\appicon.ico"

Section "Main"
	ExecShell "open" "$PROGRAMFILES64\HWiNFOPortableTemp\HWiNFO64.exe"
SectionEnd