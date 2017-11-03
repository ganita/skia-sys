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


#ifndef rs_sk_canvas_DEFINED
#define rs_sk_canvas_DEFINED

#include "sk_types.h"

SK_C_PLUS_PLUS_BEGIN_GUARD

/** Draw the text, with origin at (x,y), using the specified paint.
    The origin is interpreted based on the Align setting in the paint.
    @param ccanvas
    @param text The text to be drawn
    @param byteLength   The number of bytes to read from the text parameter
    @param x        The x-coordinate of the origin of the text being drawn
    @param y        The y-coordinate of the origin of the text being drawn
    @param paint    The paint used for the text (e.g. color, size, style)
*/
SK_API void sk_canvas_draw_text(sk_canvas_t* ccanvas, const void* text, size_t length, float x, float y, const sk_paint_t* cpaint);

SK_C_PLUS_PLUS_END_GUARD

#endif