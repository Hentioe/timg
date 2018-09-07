# TIMG

````
```````````````````````````````````````````````````````
````````````````-:----.```````.-::::--.````````...`````
```````````````sNNMMMMNh+++o+/:-------::++/:/smNNNmy:``
`````````````./NMMMMMMMd+-.````````````````-odMMMMMMNo`
````````.:+ydNMMMMMMMNy-`````````````````````.sNMMMMMd`
`````-+hmNMMMMMMMMMMm+.``````````````..```````.sNMMMMh`
``-+hNMMMMMMMMMMMMMd:```````.-::/-``/hhys/.````.sNMNh:`
/ymMMMMMMMMMMMMMMMyo.`....:shhhddh+./o++/:-.....-ss-```
NMMMMMMMMMMMMMMNmms:..-:::+sssyyo/:.`-+yo:--:::::+y-```
MMMMMMMMMMNmds+/-o+--:/++oosyys+:..```.:///++ooo++h+```
MMMMMMMMMNh/:.../o:-:+oossssoo++so::://oydyssssssoyh:``
mMMMMMMMMMMMNmmmNo.-/+ossssssyyhsoyhddyssyhysssssoyNmy:
:hNMMMMMMMMMMMMMMs.-:/+oosssyyyhyyssoo+oysooosssoosNMMy
`.+mMMMMMMMMMMMMMms:--:/+++++/+oooo+::-/+::/++++++hMMMy
```-yNMMMMMMMMMMMMNd+:---:--:::////+oo++:----::+smMMMMh
`````/hNMMMMMMMMMMMMNmh+-..````.....---....-//ymMMMMMMh
```````/ymMMMMMMMMMMMMMNmdyso+::::////++syhmNNMMMMMMMMh
```` ````-syyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyo
```-`.-````/```.----..-----`-..--..-::-.`----.`.-`:```
``:y.+h/``.N-``ymodd/-/yms/-my+od-/dddd/`mymhs`hyyyo+
```d/+y.`.hsh-`+hohho-ssm/s-hm+yy:.dhhd-.hhdsy.myooo+
```++oo+-o-`.+--++oo+``.o``.+:-o:`//oo//--o+o:.o/./o.
```````````````````````````````````````````````````````
````

使用 TIMG 将图片转为 ASCII 文本！


## Usage

Normal:

```` bash
./timg img.jpg
````

Output file: `img.jpg.html`

Limit size:

````
./timg -w 50 img.jpg
````

## Help:

```` bash
TIMG alpha
Hentioe Cl (绅士喵)
Image to ASCII

USAGE:
    timg [OPTIONS] <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    Output target: html/default: text for terminal
    -s, --scale <scale>      Scale html font size, unit: [pixel]
    -w, --width <width>      Set output ASCII text width

ARGS:
    <path>    The image file path
````

## TODO(1.0)

* [x] 基本的图片输入网页输出
* [x] 限制输出大小
* [x] 增加输出目标：文本和终端
* [x] 强制缩放网页字体大小
* [ ] 浏览器自动预览
* [ ] 添加上游：本地算法

## TODO(2.0)

* [ ] 本地 HTTP 服务器支持
* [ ] ASCII 动画
  * [ ] 输入源支持视频文件
  * [ ] 输出每一帧为文本，支持自定义帧数
  * [ ] 本地支持循环播放每一帧文本