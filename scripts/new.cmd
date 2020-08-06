echo off

IF [%1]==[] (
  echo "new branch name not supplied"
) ELSE (
  git checkout template
  git branch %1
  git checkout %1
)

echo on