read n

for row in {63..1}; do
  for col in {1..100}; do
    if [ $row -le 16 -a $col -eq 50 ]; then
      printf 1
    elif [ $row -gt 16 -a $row -le 32 ] &&
      [ $(($col - 50)) -eq $(($row - 16)) -o $((50 - $col)) -eq $(($row - 16)) ]
    then
      printf 1
    elif [ $n -gt 1 -a $row -gt 32 -a $row -le 40 ] && [ $col -eq 66 -o $col -eq 34 ]; then
      printf 1
    elif [ $n -gt 1 -a $row -gt 40 -a $row -le 48 ] &&
      [ $(($col - 34)) -eq $(($row - 40)) -o $((34 - $col)) -eq $(($row - 40)) -o \
      $(($col - 66)) -eq $(($row - 40)) -o $((66 - $col)) -eq $(($row - 40)) ]
    then
      printf 1
    elif [ $n -gt 2 -a $row -gt 48 -a $row -le 52 ] && [ $col -eq 26 -o $col -eq 42 -o $col -eq 58 -o $col -eq 74 ]
    then
      printf 1
    elif [ $n -gt 2 -a $row -gt 52 -a $row -le 56 ] &&
      [ $(($col - 26)) -eq $(($row - 52)) -o $((26 - $col)) -eq $(($row - 52)) -o \
      $(($col - 42)) -eq $(($row - 52)) -o $((42 - $col)) -eq $(($row - 52)) -o \
      $(($col - 58)) -eq $(($row - 52)) -o $((58 - $col)) -eq $(($row - 52)) -o \
      $(($col - 74)) -eq $(($row - 52)) -o $((74 - $col)) -eq $(($row - 52)) ]
    then
      printf 1
  elif [ $n -gt 3 -a $row -gt 56 -a $row -le 58 ] && [ $(( ($col - 22) % 8 )) -eq 0 -a $col -ge 22 -a $col -le 78 ]
    then
      printf 1
    elif [ $n -gt 3 -a $row -gt 58 -a $row -le 60 ] &&
      [ $(($col - 22)) -eq $(($row - 58)) -o $((22 - $col)) -eq $(($row - 58)) -o \
      $(($col - 30)) -eq $(($row - 58)) -o $((30 - $col)) -eq $(($row - 58)) -o \
      $(($col - 38)) -eq $(($row - 58)) -o $((38 - $col)) -eq $(($row - 58)) -o \
      $(($col - 46)) -eq $(($row - 58)) -o $((46 - $col)) -eq $(($row - 58)) -o \
      $(($col - 54)) -eq $(($row - 58)) -o $((54 - $col)) -eq $(($row - 58)) -o \
      $(($col - 62)) -eq $(($row - 58)) -o $((62 - $col)) -eq $(($row - 58)) -o \
      $(($col - 70)) -eq $(($row - 58)) -o $((70 - $col)) -eq $(($row - 58)) -o \
      $(($col - 78)) -eq $(($row - 58)) -o $((78 - $col)) -eq $(($row - 58)) ]
    then
      printf 1
  elif [ $n -gt 4 -a $row -eq 61 ] && [ $(( ($col - 20) % 4 )) -eq 0 -a $col -ge 20 -a $col -le 80 ]
    then
      printf 1
  elif [ $n -gt 4 -a $row -eq 62 ] && [ $(( ($col - 20) % 4 )) -eq 1 -o $(( ($col - 20) % 4 )) -eq 3 -o $col -eq 19 ] && [ $col -ge 16 -a $col -le 82 ]
    then
      printf 1
    else
      printf _
    fi
  done
  echo
done
