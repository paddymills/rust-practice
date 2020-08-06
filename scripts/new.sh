if [$1 -eq "" ] 
then
  echo "new branch name not supplied"
else
  git checkout template
  git branch $1
  git checkout $1
fi
