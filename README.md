# Imagen

Framework for building images from rects and circles.

This project contains framework itself, and demo api and web app

## **Framework description:**
_(core and img folder)_

I built this framework as much from scratch as i could. I build shapes pixel by pixel in my own canvas implementation. I handle png building myself, exept for compression algorithm part (i use external create for that). I also used extenral create in text generation to decode font glyphs.

### **Functions:**
 - drawing circles and rects
 - drawing these shapes rouned
 - drawing only outlines of shapes
 - filling outlines with different colors
 - drawing text
 
### **Technologies:** 
Built with rust and these libs:
- ab_glyph - decoding fonts
- crc32fast - png checksums
- flate2 - compression algorithm


## **Simple code example demo:**
_(demo folder)_

In this folder there is simple code that generates a few images to output folder. Nothing more nothing less.

## **Api + frontend:**
_(api and web folder)_

### **Functions:**
- simple landing page with links to demo and github
- live demo with image creating with blocks
- downloading generated images
- generating all whats possible using imagen framework

### **Openapi documentation:**

You can view all the enpoints that this api has [here](https://imagen.piernik.rocks/docs/scalar)

### **Technologies:**

I built this project's frontend partialy with [shadcn](https://ui.shadcn.com/) components

### **Host it yourself:**

You can host it by yourself with docker!

_If you want to use something different than traefik, you need to adjust compose file for your needs._

**Dependencies:**
- [docker](https://docs.docker.com/engine/install/)
- [traefik](https://doc.traefik.io/traefik/getting-started/install-traefik/)

### **Run:**

```bash
docker-compose up -d
```

### **Development:**

**Dependencies:**

- [pnpm](https://pnpm.io/installation)
- [rust/cargo](https://rust-lang.org/tools/install/)
- [caddy](https://caddyserver.com/)

**Run:**

**web**

```bash
cd web/
pnpm i
pnpm dev
```

**api**

```bash
cd api/
cargo run
```

**caddy** - mixing api and web together on one port

```bash
caddy run
```
