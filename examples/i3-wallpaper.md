# Dynamic wallpaper
This example shows how to update wallpaper every time you change theme.

> Note: this example is based on `i3.md` example, so take a look there to understand what's happening.

> Note 2: in this example I show configuration specifically for i3 wallpaper, but you can use this with any config file such as sway or something else.

# Setup
If you don't have any Themer blocks inside your i3, you can can just wrap code where you set your wallpaper with Themer block and skip to the next section.  
If you've set some block that defines other things inside your config, you should tag your config.
> Note: Tags allows themer to manage your config in multiple places, so you can separate logic.  

So if you had some block that defines colors, you should do this:
```diff
- # THEMER 
- # THEMER_END
+ # THEMER:colors
+ # THEMER_END:colors
```
> Note: `colors` is abritrary single-word tag name 

Now let's add another block for changing wallpaper.
```diff
+ # THEMER:wallpaper
+ # THEMER_END:wallpaper
```

# Themer config
Now we need to patch your config a little bit (this one is based on config from `i3.md` example):
```yaml
# ~./config/themer/config.yml
files:
  i3:
    # Themer can expand tildes!
    path: "~/.config/i3/config"
    comment: "#" # you can skip this one, because hash (#) is the default comment for Themer
    # Here we define blocks inside our file
    blocks:
      colors:  
        format: "set $<key> <value>"
      wallpaper:
        # Here we define a code that will be inserted inside `THEMER:wallpaper` block
        # I use feh to set background, you can use any other tool
        # Also note <name>.png part, this is a dynamic varialbe that contains name of the current theme
        custom: |
          exec_always  feh --bg-scale /home/user/wallpapers/<name>.png
```

# Images
To match expression insde `wallpaper.custom`, we should create files inside `/home/user/wallpapers/` that 
match names of themes defined inside Themer's `config.yml`. Let's assume you have 2 themes: `dark` and `light`.  

So you need to have these files:
```bash
$ ls /home/user/wallpapers
> dark.png
> light.png
```

# Checking
You can additionally check wether Themer recognizes blocks inside config file:
```bash
$ themer files --check
> Listed configuration files:
>
> i3 (~/.config/i3/config) [Multiple blocks]:
>  ok colors
>  ok wallpaper
```

# Result 
Now, when you run `themer set <theme name>` you'll also get automatically updated wallpaper that matches your color scheme.
