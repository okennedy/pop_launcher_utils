
PLUGINS="todo pass"

cargo build

for plug in $PLUGINS; do
  DIR=~/.local/share/pop-launcher/plugins/$plug
  BIN=target/debug/$plug
  RON=./$plug.ron

  if [ ! -d $DIR ] ; then
    echo Creating $DIR
    mkdir -p $DIR
  fi
  echo Installing $RON "->" $DIR/plugin.ron
  cp $RON $DIR/plugin.ron
  echo Installing $BIN "->" $DIR
  cp $BIN $DIR
done
