/*
 * Copyright 2017 Sreejith Krishnan R
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/


#include "SkCanvas.h"
#include "rs_sk_canvas.h"
#include "sk_types_priv.h"

static SkCanvas* AsCanvas(sk_canvas_t* ccanvas) {
    return reinterpret_cast<SkCanvas*>(ccanvas);
}

void sk_canvas_draw_text(sk_canvas_t* ccanvas, const void* text, size_t length, float x, float y, const sk_paint_t* cpaint) {
    AsCanvas(ccanvas)->drawText(text, length, x, y, AsPaint(*cpaint));
}