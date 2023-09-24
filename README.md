## Use-case
### Use Filebeat to collect Mac OS logs and send them to Elasticsearch.
In a log stream, one log is written in multiple lines, but due to the filestream nature of filebeat, it recognizes and sends one log as a single line, so the above file is required.

Create a log by filtering only certain events
```
log stream --predicate '(subsystem == "com.apple.sharing" && category == "AirDrop") || (process == "apsd" && subsystem == "com.apple.apsd") || (subsystem == "com.apple.sharing" && category == "AutoUnlock") || (process == "softwareupdated" && (subsystem == "com.apple.mac.install" || subsystem == "com.apple.SoftwareUpdate" || subsystem == "com.apple.SoftwareUpdateMacController" || subsystem == "com.apple.mobileassetd")) || (subsystem == "com.apple.network" && (category == "connection" || category == "boringssl")) || (process == "opendirectoryd" && (subsystem == "com.apple.opendirectoryd" || subsystem == "com.apple.AccountPolicy")) || (process == "loginwindow" && subsystem == "com.apple.login")' | ~/Documents/EDR-LOG/bin/single-line ~ >> ~/Documents/EDR-LOG/log/mac-end.log
```
Log records for AirDrop, Apple Push Notification Service (APNs), Apple Watch unlock, macOS Installer and Software Update, Networking, Open Directory, User login
</br></br>

#### Copyright 2023. ClumL Inc. all rights reserved