$username = Read-Host "Please enter username"
$password = Read-Host "Please enter password"
$Env:VAULT_ADDR = "https://civ1.dv.adskengineer.net:8200"
C:/Hashicorp/vault.exe login -method=ldap username=$username password=$password
vault write account/865412956762/sts/Owner ttl=30m | Out-File -Encoding "UTF8" -FilePath C:\Users\sean.barnard\.aws\vault.txt