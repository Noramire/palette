# palette

An application written on Rust that gets a color palette as a beautiful web page!

![image](palette_examples.jpg)

## Example

```powershell
./palette.exe -s folder/image.jpg -c 8
```

**Result:** exports *index.html* in the directory with your image.

## Image Exporting
Depends on which algorithm you choose, application can export a quantized image. (If you use **"Common Pixels with a dictionary"** algorithm, it will sort pixels in descending order).

## Commands

```
palette 0.1.1

USAGE:
    palette.exe [OPTIONS] --src <SRC>

OPTIONS:
    -c, --colors <COLORS>    Number of shades you want to get [default: 8]
    -h, --help               Print help information
        --method <METHOD>    Quantization method
                             0. Median Cut Algorithm
                             1. Common Pixels with a dictionary
                                  [default: 0]
        --no-export          Don't export a web page
    -o, --out <OUT>          Export an image (e.g assets/new_img.jpg) [default: ]
    -s, --src <SRC>          Source image (e.g. assets/img.jpg)
        --silent             Don't print an array of palette's shades
    -V, --version            Print version information
```