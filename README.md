# TIMG

                    `/#N#####h`   ``sshhhhhhhhhhhsss`    `s///hs`
                    /MMMMMMMMMM#//hhs`            `shhhh#MMMMMMMM/`
                    sMMMMMMMMMMNs`                      s/NMMMMMMMM#
                `h/NMMMMMMMMMMM#`                          s#MMMMMMMMs
            `/#MMMMMMMMMMMMMMNh                             `#MMMMMMMs
        `/NMMMMMMMMMMMMMMMM#`                  `sss``       sNMMMMM#`
    `h#MMMMMMMMMMMMMMMMMMNs         `ssh//s  `/NNN##h`      s#MMN/`
    s#NMMMMMMMMMMMMMMMMMMM//`      `h#NNN#NNN/``shss`` `` ``` `h#s
    MMMMMMMMMMMMMMMMMMMMMN/s `````ssh/hh//#/hh`  s/N#h````ssss`s/h
    MMMMMMMMMMMMMMMMMN#hs//```sshhhh/##NN#/`       ````sshhhhhhs/#s
    MMMMMMMMMMMMN/ss    shs``shh////////hssss`    `sh//hh//////h/Nh
    MMMMMMMMMMMMN/hs```s#h``shh////##///hh/#/ssshhh//#NN#####////NNh
    NMMMMMMMMMMMMMMMMMMMMs``shh///#######N#hh/##NN#/h/########///#MM#h
    sNMMMMMMMMMMMMMMMMMMNs``ssh//////#######//##///h/#///////////#MMMMs
    hMMMMMMMMMMMMMMMMMM/s``sshhh///hhhh///##hss`sh/hsshh////hhhNMMMMs
    `#MMMMMMMMMMMMMMMMMNh```ssssssssssshhssss````s``ssssssssh#MMMMMs
        hNMMMMMMMMMMMMMMMMM#ss```````````sssshh///hh```````sh#NMMMMMMs
        `/MMMMMMMMMMMMMMMMMMM#h`          `    ``      `ss#MMMMMMMMMs
            `/NMMMMMMMMMMMMMMMMMMN#/hss``        ```ssh/#NMMMMMMMMMMMMs
            `hNMMMMMMMMMMMMMMMMMMMMMMMNNNNNNNNMMMMMMMMMMMMMMMMMMMMMMs
                `````````````````````````````````````````````````````

使用 TIMG 将图片转为 ASCII 文本！

## Usage

```` bash
# Local file
timg ./res/img.jpg

# Remote file
timg https://www.baidu.com/img/bd_logo1.png
````

## Help

```` text
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
* [x] 支持网络路径
* [x] 浏览器自动预览
* [x] 添加上游：本地算法

## TODO(2.0)

* [ ] 本地 HTTP 服务器支持
* [ ] ASCII 动画
  * [ ] 输入源支持视频文件
  * [ ] 输出每一帧为文本，支持自定义帧数
  * [ ] 本地支持循环播放每一帧文本
