/*
 * Spinning 3D version of the Cal x Stanford chip on the homepage.
 *
 * The static <img> (marked data-chip-spin) always stays in the page. This script only lays a
 * WebGL canvas over it once the 3D chip has drawn successfully, and puts the image back on any
 * problem. It never runs on touch-only devices, with reduced motion, with data saver, or
 * without WebGL, so those visitors keep the static chip.
 *
 * The homepage calls window.PabChipSpin.mount(img) after rendering. If this script loads
 * later than that, it mounts any marked image it finds on its own.
 *
 * Plain WebGL 1, no libraries, so it stays small.
 */
(function () {
  "use strict";

  // ============================================================
  // Config
  // ============================================================

  // Face images, and where the chip's circle sits in each (image pixels, measured).
  // The static homepage image is the same file as chip-front.png, so FRONT also says where the
  // chip is inside the <img>. If that image changes, re-measure.
  var FRONT = { src: "/public/images/chip-spin/chip-front.webp", size: 525, cx: 261.67, cy: 263.49, r: 241.04 };
  var BACK = { src: "/public/images/chip-spin/chip-back.webp", size: 1254, cx: 619.72, cy: 627.94, r: 573.58 };
  var FACE_TEXTURE_SIZE = 1024; // both faces are scaled to this, so they match
  var FACE_INSET_PX = 1.5;      // trims the antialiased edge pixels (source image pixels)

  var THICKNESS = 0.16;         // chip radius is 1, so this is 12.5:1 diameter to thickness
  var EDGE_ROUNDING = 0.45;     // how rounded the edge looks in the lighting (0 = flat side)

  // Edge colors, sampled from the face images.
  var RED = "#8c1217";
  var BLUE = "#0a3260";
  var WHITE = "#fdfdfd";
  var GOLD = "#fdc51e";

  // Angles below: degrees, 0 = 3 o'clock, counterclockwise, as seen from the front.
  // The red half runs from RED_FROM to RED_TO. The back's split is about 2 degrees off the
  // front's, so the color boundary slants slightly across the edge to meet both faces.
  var RED_FROM = 46.75;
  var RED_TO_FRONT = 228.75;
  var RED_TO_BACK = 226.5;
  // Edge spots, centered on the dashed stripes of both faces (the average of the two).
  var EDGE_SPOTS = [24.4, 68.1, 112.9, 157.2, 201.8, 249.5, 294.8, 339.8];
  var EDGE_SPOT_WIDTH = 23.3;   // degrees, the same as the stripes
  var EDGE_SPOT_HEIGHT = 0.62;  // fraction of the chip's thickness

  // Motion
  var MAX_TURN_DEG = 180;       // mouse at either screen edge shows the back
  var MAX_TILT_DEG = 25;        // mouse at the top or bottom edge
  var SMOOTH_TIME = 0.32;       // seconds to roughly catch up with the mouse; higher = lazier

  // Lighting. Facing straight at you, the brightness is exactly 1, so the colors match the image.
  var LIGHT_DIR = [-0.35, 0.45, 0.82]; // from the upper left, in front
  var AMBIENT = 0.62;

  var PADDING = 0.06;           // extra canvas around the chip, so the edge isn't clipped while turning
  var CAMERA_DISTANCE = 6;      // lower = stronger perspective
  var MAX_PIXEL_RATIO = 2;

  // ============================================================
  // When to run
  // ============================================================

  function query(q) {
    return window.matchMedia ? window.matchMedia(q) : null;
  }

  function canRun() {
    var mouse = query("(hover: hover) and (pointer: fine)");
    var reduced = query("(prefers-reduced-motion: reduce)");
    if (!mouse || !mouse.matches || !reduced || reduced.matches) return false;
    var connection = navigator.connection;
    if (connection && connection.saveData) return false;
    return typeof window.requestAnimationFrame === "function" &&
      typeof window.ResizeObserver === "function" &&
      typeof window.Promise === "function" &&
      typeof window.Float32Array === "function";
  }

  // Wait for the rest of the page first, so the chip never slows anything else down.
  function whenIdle(fn) {
    var go = function () {
      if (window.requestIdleCallback) window.requestIdleCallback(fn, { timeout: 2000 });
      else setTimeout(fn, 200);
    };
    if (document.readyState === "complete") go();
    else window.addEventListener("load", go, { once: true });
  }

  function loadImage(src) {
    return new Promise(function (resolve, reject) {
      var image = new Image();
      image.onload = function () { resolve(image); };
      image.onerror = function () { reject(new Error("could not load " + src)); };
      image.src = src;
    });
  }

  // ============================================================
  // Small matrix helpers (column-major, as WebGL expects)
  // ============================================================

  function multiply(a, b) {
    var out = new Float32Array(16);
    for (var col = 0; col < 4; col++) {
      for (var row = 0; row < 4; row++) {
        var sum = 0;
        for (var k = 0; k < 4; k++) sum += a[k * 4 + row] * b[col * 4 + k];
        out[col * 4 + row] = sum;
      }
    }
    return out;
  }

  function perspective(tanHalfFov, near, far) {
    var f = 1 / tanHalfFov;
    return new Float32Array([
      f, 0, 0, 0,
      0, f, 0, 0,
      0, 0, (far + near) / (near - far), -1,
      0, 0, (2 * far * near) / (near - far), 0,
    ]);
  }

  function rotationX(a) {
    var c = Math.cos(a), s = Math.sin(a);
    return new Float32Array([1, 0, 0, 0, 0, c, s, 0, 0, -s, c, 0, 0, 0, 0, 1]);
  }

  function rotationY(a) {
    var c = Math.cos(a), s = Math.sin(a);
    return new Float32Array([c, 0, -s, 0, 0, 1, 0, 0, s, 0, c, 0, 0, 0, 0, 1]);
  }

  function upperLeft3x3(m) {
    return new Float32Array([m[0], m[1], m[2], m[4], m[5], m[6], m[8], m[9], m[10]]);
  }

  // ============================================================
  // Textures, drawn on 2D canvases first
  // ============================================================

  // Crops the chip's circle out of a face image into a square texture. The corners are filled
  // with the chip's own edge colors (the image enlarged), so the transparent background can't
  // bleed into the edge when the texture is shrunk.
  function faceCanvas(image, face) {
    var size = FACE_TEXTURE_SIZE;
    var scale = image.naturalWidth / face.size; // in case a larger copy is ever swapped in
    var r = (face.r - FACE_INSET_PX) * scale;
    var x = face.cx * scale - r, y = face.cy * scale - r;
    var canvas = document.createElement("canvas");
    canvas.width = canvas.height = size;
    var g = canvas.getContext("2d");
    g.imageSmoothingQuality = "high";
    var grown = size * Math.SQRT2, offset = (size - grown) / 2;
    g.drawImage(image, x, y, r * 2, r * 2, offset, offset, grown, grown);
    g.beginPath();
    g.arc(size / 2, size / 2, size / 2, 0, Math.PI * 2);
    g.clip();
    g.drawImage(image, x, y, r * 2, r * 2, 0, 0, size, size);
    return canvas;
  }

  // The edge, unrolled: x runs once around the chip (0 to 360 degrees), y runs from the front
  // face (top) to the back face (bottom).
  function edgeCanvas() {
    var w = 2048, h = 64;
    var canvas = document.createElement("canvas");
    canvas.width = w;
    canvas.height = h;
    var g = canvas.getContext("2d");
    var x = function (deg) { return (deg / 360) * w; };

    g.fillStyle = BLUE;
    g.fillRect(0, 0, w, h);
    g.fillStyle = RED;
    g.beginPath();
    g.moveTo(x(RED_FROM), 0);
    g.lineTo(x(RED_TO_FRONT), 0);
    g.lineTo(x(RED_TO_BACK), h);
    g.lineTo(x(RED_FROM), h);
    g.closePath();
    g.fill();

    var spotTop = h * (1 - EDGE_SPOT_HEIGHT) / 2;
    EDGE_SPOTS.forEach(function (center) {
      var onRed = center > RED_FROM && center < (RED_TO_FRONT + RED_TO_BACK) / 2;
      g.fillStyle = onRed ? WHITE : GOLD;
      g.fillRect(x(center - EDGE_SPOT_WIDTH / 2), spotTop, x(EDGE_SPOT_WIDTH), h * EDGE_SPOT_HEIGHT);
    });
    return canvas;
  }

  // ============================================================
  // Geometry: front disc, back disc, and the edge band
  // ============================================================

  var SEGMENTS = 192;

  // Each vertex: x, y, z, normal x, y, z, u, v.
  function discGeometry(front) {
    var z = (front ? 1 : -1) * THICKNESS / 2;
    var nz = front ? 1 : -1;
    var mirror = front ? 1 : -1; // the back is mirrored so it reads correctly once turned around
    var data = [0, 0, z, 0, 0, nz, 0.5, 0.5];
    for (var i = 0; i <= SEGMENTS; i++) {
      var a = (i / SEGMENTS) * Math.PI * 2;
      var x = Math.cos(a), y = Math.sin(a);
      data.push(x, y, z, 0, 0, nz, 0.5 + (mirror * x) / 2, 0.5 - y / 2);
    }
    return { data: new Float32Array(data), mode: "TRIANGLE_FAN", count: SEGMENTS + 2 };
  }

  function edgeGeometry() {
    var data = [];
    var half = THICKNESS / 2;
    for (var i = 0; i <= SEGMENTS; i++) {
      var a = (i / SEGMENTS) * Math.PI * 2;
      var x = Math.cos(a), y = Math.sin(a), u = i / SEGMENTS;
      // Normals tip toward each face, so the edge lights like a rounded one.
      var n1 = normalize([x, y, EDGE_ROUNDING]);
      var n2 = normalize([x, y, -EDGE_ROUNDING]);
      data.push(x, y, half, n1[0], n1[1], n1[2], u, 0);
      data.push(x, y, -half, n2[0], n2[1], n2[2], u, 1);
    }
    return { data: new Float32Array(data), mode: "TRIANGLE_STRIP", count: (SEGMENTS + 1) * 2 };
  }

  function normalize(v) {
    var l = Math.hypot(v[0], v[1], v[2]);
    return [v[0] / l, v[1] / l, v[2] / l];
  }

  // ============================================================
  // Shaders: matte plastic, a soft key light, and an ambient term
  // ============================================================

  var VERTEX_SHADER = [
    "attribute vec3 aPosition;",
    "attribute vec3 aNormal;",
    "attribute vec2 aUv;",
    "uniform mat4 uModelViewProjection;",
    "uniform mat3 uRotation;",
    "varying vec3 vNormal;",
    "varying vec2 vUv;",
    "void main() {",
    "  vNormal = uRotation * aNormal;",
    "  vUv = aUv;",
    "  gl_Position = uModelViewProjection * vec4(aPosition, 1.0);",
    "}",
  ].join("\n");

  var FRAGMENT_SHADER = [
    "precision mediump float;",
    "uniform sampler2D uTexture;",
    "uniform vec3 uLight;",
    "uniform float uAmbient;",
    "uniform float uDiffuse;",
    "varying vec3 vNormal;",
    "varying vec2 vUv;",
    "void main() {",
    "  vec3 n = normalize(vNormal);",
    "  float light = uAmbient + uDiffuse * max(dot(n, uLight), 0.0);",
    "  gl_FragColor = vec4(texture2D(uTexture, vUv).rgb * light, 1.0);",
    "}",
  ].join("\n");

  function compile(gl, type, source) {
    var shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) throw new Error(gl.getShaderInfoLog(shader));
    return shader;
  }

  // ============================================================
  // Mounting
  // ============================================================

  function mount(img) {
    if (!img || img.getAttribute("data-chip-spin-state")) return;
    if (!canRun()) return img.setAttribute("data-chip-spin-state", "off");
    img.setAttribute("data-chip-spin-state", "waiting");
    whenIdle(function () {
      if (!img.isConnected) return;
      try {
        start(img);
      } catch (error) {
        fail(error);
      }
    });
  }

  function fail(error) {
    if (window.console) console.warn("[chip-spin] showing the static chip:", error);
  }

  function start(img) {
    var canvas = document.createElement("canvas");
    canvas.setAttribute("aria-hidden", "true");
    canvas.style.cssText = "position:absolute;pointer-events:none;";

    var gl = canvas.getContext("webgl", {
      alpha: true,
      premultipliedAlpha: true,
      antialias: true,
      depth: true,
      failIfMajorPerformanceCaveat: true, // no software rendering
    });
    if (!gl) return fail(new Error("WebGL is not available"));

    var stopped = false;
    var cleanups = [];
    function listen(target, type, fn, options) {
      target.addEventListener(type, fn, options);
      cleanups.push(function () { target.removeEventListener(type, fn, options); });
    }

    // Removes the canvas and shows the static image again. Safe to call more than once.
    function stop(error) {
      if (stopped) return;
      stopped = true;
      if (frameId) cancelAnimationFrame(frameId);
      cleanups.forEach(function (fn) { fn(); });
      img.style.opacity = "";
      if (canvas.parentNode) canvas.parentNode.removeChild(canvas);
      var lose = gl.getExtension("WEBGL_lose_context");
      if (lose) lose.loseContext();
      img.setAttribute("data-chip-spin-state", error ? "failed" : "stopped");
      if (error) fail(error);
    }

    // ---- Program and geometry ----
    var program = gl.createProgram();
    gl.attachShader(program, compile(gl, gl.VERTEX_SHADER, VERTEX_SHADER));
    gl.attachShader(program, compile(gl, gl.FRAGMENT_SHADER, FRAGMENT_SHADER));
    gl.linkProgram(program);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) throw new Error(gl.getProgramInfoLog(program));
    gl.useProgram(program);

    var at = {
      position: gl.getAttribLocation(program, "aPosition"),
      normal: gl.getAttribLocation(program, "aNormal"),
      uv: gl.getAttribLocation(program, "aUv"),
    };
    var un = {
      mvp: gl.getUniformLocation(program, "uModelViewProjection"),
      rotation: gl.getUniformLocation(program, "uRotation"),
      light: gl.getUniformLocation(program, "uLight"),
      ambient: gl.getUniformLocation(program, "uAmbient"),
      diffuse: gl.getUniformLocation(program, "uDiffuse"),
      texture: gl.getUniformLocation(program, "uTexture"),
    };

    var light = normalize(LIGHT_DIR);
    gl.uniform3fv(un.light, light);
    gl.uniform1f(un.ambient, AMBIENT);
    gl.uniform1f(un.diffuse, (1 - AMBIENT) / light[2]); // facing you, brightness is exactly 1
    gl.uniform1i(un.texture, 0);

    function upload(geometry) {
      var buffer = gl.createBuffer();
      gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
      gl.bufferData(gl.ARRAY_BUFFER, geometry.data, gl.STATIC_DRAW);
      geometry.buffer = buffer;
      return geometry;
    }
    var parts = [upload(discGeometry(true)), upload(discGeometry(false)), upload(edgeGeometry())];

    var anisotropy = gl.getExtension("EXT_texture_filter_anisotropic") ||
      gl.getExtension("WEBKIT_EXT_texture_filter_anisotropic");

    function texture(source) {
      var tex = gl.createTexture();
      gl.bindTexture(gl.TEXTURE_2D, tex);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, source);
      gl.generateMipmap(gl.TEXTURE_2D);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      if (anisotropy) {
        var max = gl.getParameter(anisotropy.MAX_TEXTURE_MAX_ANISOTROPY_EXT);
        gl.texParameterf(gl.TEXTURE_2D, anisotropy.TEXTURE_MAX_ANISOTROPY_EXT, Math.min(8, max));
      }
      return tex;
    }

    gl.enable(gl.DEPTH_TEST);
    gl.clearColor(0, 0, 0, 0);

    // ---- Size and position, matched to the static image ----
    var tanHalfFov = (1 + PADDING) / (CAMERA_DISTANCE - THICKNESS / 2); // front face fills the disc exactly
    var projection = perspective(tanHalfFov, 0.1, 20);
    var view = new Float32Array([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, -CAMERA_DISTANCE, 1]);
    var viewProjection = multiply(projection, view);

    function layout() {
      var width = img.offsetWidth;
      if (!width) return false;
      var scale = width / FRONT.size;
      var side = FRONT.r * scale * 2 * (1 + PADDING);
      canvas.style.left = img.offsetLeft + FRONT.cx * scale - side / 2 + "px";
      canvas.style.top = img.offsetTop + FRONT.cy * scale - side / 2 + "px";
      canvas.style.width = canvas.style.height = side + "px";
      var pixels = Math.round(side * Math.min(window.devicePixelRatio || 1, MAX_PIXEL_RATIO));
      if (canvas.width !== pixels) {
        canvas.width = canvas.height = pixels;
        gl.viewport(0, 0, pixels, pixels);
      }
      return true;
    }

    // ---- Motion: a critically damped spring per axis (smooth, no overshoot) ----
    var turn = { x: 0, v: 0, target: 0 };
    var tilt = { x: 0, v: 0, target: 0 };

    function ease(s, dt) {
      var omega = 2 / SMOOTH_TIME;
      var k = omega * dt;
      var decay = 1 / (1 + k + 0.48 * k * k + 0.235 * k * k * k);
      var offset = s.x - s.target;
      var pull = (s.v + omega * offset) * dt;
      s.v = (s.v - omega * pull) * decay;
      s.x = s.target + (offset + pull) * decay;
    }

    function settled(s) {
      return Math.abs(s.x - s.target) < 1e-4 && Math.abs(s.v) < 1e-4;
    }

    var textures = null;

    function draw() {
      var rotation = multiply(rotationX(tilt.x), rotationY(turn.x));
      gl.uniformMatrix4fv(un.mvp, false, multiply(viewProjection, rotation));
      gl.uniformMatrix3fv(un.rotation, false, upperLeft3x3(rotation));
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
      for (var i = 0; i < parts.length; i++) {
        var part = parts[i];
        gl.bindBuffer(gl.ARRAY_BUFFER, part.buffer);
        gl.enableVertexAttribArray(at.position);
        gl.vertexAttribPointer(at.position, 3, gl.FLOAT, false, 32, 0);
        gl.enableVertexAttribArray(at.normal);
        gl.vertexAttribPointer(at.normal, 3, gl.FLOAT, false, 32, 12);
        gl.enableVertexAttribArray(at.uv);
        gl.vertexAttribPointer(at.uv, 2, gl.FLOAT, false, 32, 24);
        gl.bindTexture(gl.TEXTURE_2D, textures[i]);
        gl.drawArrays(gl[part.mode], 0, part.count);
      }
      var error = gl.getError();
      if (error !== gl.NO_ERROR && error !== gl.CONTEXT_LOST_WEBGL) throw new Error("WebGL error " + error);
    }

    var frameId = 0;
    var lastTime = 0;
    var onScreen = true;

    function frame(time) {
      frameId = 0;
      if (stopped) return;
      if (!img.isConnected) return stop();
      try {
        var dt = lastTime ? Math.min((time - lastTime) / 1000, 0.05) : 1 / 60;
        lastTime = time;
        ease(turn, dt);
        ease(tilt, dt);
        draw();
        if (!(settled(turn) && settled(tilt))) wake(true);
        else lastTime = 0;
      } catch (error) {
        stop(error);
      }
    }

    function wake(continuing) {
      if (stopped || frameId || !textures || (!onScreen && !continuing)) return;
      frameId = requestAnimationFrame(frame);
    }

    // ---- Input ----
    var DEG = Math.PI / 180;
    listen(window, "pointermove", function (e) {
      if (e.pointerType === "touch") return;
      var nx = (e.clientX / window.innerWidth) * 2 - 1;  // -1 left edge, 1 right edge
      var ny = (e.clientY / window.innerHeight) * 2 - 1; // -1 top, 1 bottom
      turn.target = Math.max(-1, Math.min(1, nx)) * MAX_TURN_DEG * DEG;
      tilt.target = Math.max(-1, Math.min(1, ny)) * MAX_TILT_DEG * DEG;
      wake();
    }, { passive: true });

    function faceFront() {
      turn.target = 0;
      tilt.target = 0;
      wake();
    }
    listen(document, "mouseout", function (e) {
      if (!e.relatedTarget) faceFront(); // the pointer left the window
    });
    listen(window, "blur", faceFront);

    // Give the static image back if the setting or device changes while the page is open.
    [query("(prefers-reduced-motion: reduce)"), query("(hover: hover) and (pointer: fine)")].forEach(function (mq) {
      var onChange = function () { if (!canRun()) stop(); };
      if (mq.addEventListener) listen(mq, "change", onChange);
    });

    listen(canvas, "webglcontextlost", function (e) {
      e.preventDefault();
      stop(new Error("WebGL context lost"));
    });

    var resizer = new ResizeObserver(function () {
      if (stopped || !textures) return;
      try {
        if (layout()) draw();
      } catch (error) {
        stop(error);
      }
    });
    cleanups.push(function () { resizer.disconnect(); });

    if (window.IntersectionObserver) {
      var watcher = new IntersectionObserver(function (entries) {
        onScreen = entries[0].isIntersecting;
        if (onScreen) wake();
      });
      watcher.observe(img);
      cleanups.push(function () { watcher.disconnect(); });
    }

    // ---- Load the face images, then swap the canvas in ----
    img.setAttribute("data-chip-spin-state", "loading");
    Promise.all([loadImage(FRONT.src), loadImage(BACK.src)]).then(function (images) {
      if (stopped) return;
      if (!img.isConnected) return stop();
      textures = [
        texture(faceCanvas(images[0], FRONT)),
        texture(faceCanvas(images[1], BACK)),
        texture(edgeCanvas()),
      ];
      if (!layout()) throw new Error("the chip image has no size");
      img.parentNode.insertBefore(canvas, img.nextSibling);
      draw();
      // Only now that the 3D chip has drawn is the static one hidden. It stays in the page
      // (and keeps its alt text), just transparent.
      img.style.opacity = "0";
      img.setAttribute("data-chip-spin-state", "running");
      resizer.observe(img);
      wake(); // ease toward the mouse, in case it moved while the images were loading
    }).catch(stop);
  }

  // ============================================================
  // Entry points
  // ============================================================

  function mountAll() {
    var images = document.querySelectorAll("img[data-chip-spin]");
    for (var i = 0; i < images.length; i++) mount(images[i]);
  }

  window.PabChipSpin = { mount: mount };
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", mountAll);
  else mountAll();
})();
