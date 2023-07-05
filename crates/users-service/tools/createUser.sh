#!sh
set -e
INSECURE=false
while getopts "ku:p:e:" opt; do

  case "$opt" in
    k)
        INSECURE=true
        ;;
    u)
        USER="$OPTARG"
        ;;
    p)
        PASSWORD="$OPTARG"
        ;;
    e)
        EMAIL="$OPTARG"
        ;;
    --)
        shift;
        break
        ;;
  esac
done

if [[ $INSECURE == true ]]
then
  curl http://localhost:8080/user/create -XPOST --fail-with-body -H "Content-Type: application/json" -d "
  {
    \"email\": \"$EMAIL\",
    \"username\": \"$USER\",
    \"password\": \"$PASSWORD\"
  }
  "
else
  curl https://localhost:8080/user/create -XPOST --fail-with-body -H "Content-Type: application/json" -d "
  {
    \"email\": \"$EMAIL\",
    \"username\": \"$USER\",
    \"password\": \"$PASSWORD\"
  }
  "
fi


