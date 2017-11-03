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


#ifndef rs_sk_paint_DEFINED
#define rs_sk_paint_DEFINED

#include "sk_types.h"
#include "rs_sk_types.h"

SK_C_PLUS_PLUS_BEGIN_GUARD

/** Helper for setFlags(), setting or clearing the kDither_Flag bit
    @param dither   true to enable dithering, false to disable it
    */
SK_API void sk_paint_set_dither(sk_paint_t*, bool);

/** Helper for getFlags(), returning true if kDither_Flag bit is set
    @return true if the dithering bit is set in the paint's flags.
    */
SK_API bool sk_paint_is_dither(sk_paint_t*);

/** Set the paint's text size. This value must be > 0
    @param textSize set the paint's text size.
*/
SK_API void sk_paint_set_text_size(sk_paint_t*, float);

/** Return the paint's text size.
    @return the paint's text size.
*/
SK_API float sk_paint_get_text_size(sk_paint_t*);

/** Set or clear the typeface object.
    <p />
    Pass NULL to clear any previous typeface.
    As a convenience, the parameter passed is also returned.
    If a previous typeface exists, its reference count is decremented.
    If typeface is not NULL, its reference count is incremented.
    @param typeface May be NULL. The new typeface to be installed in the
                    paint
    @return         void
*/
SK_API void sk_paint_set_typeface(sk_paint_t*, sk_typeface_t* typeface);

/** Return the recommend spacing between lines (which will be
    fDescent - fAscent + fLeading).
    If metrics is not null, return in it the font metrics for the
    typeface/pointsize/etc. currently set in the paint.
    @param metrics      If not null, returns the font metrics for the
                        current typeface/pointsize/etc setting in this
                        paint.
    @param scale        If not 0, return width as if the canvas were scaled
                        by this value
    @param return the recommended spacing between lines
*/
SK_API float sk_paint_get_font_metrics(sk_paint_t*, sk_font_metrics_t* metrics, float scale);

/** Return the width of the text. This will return the vertical measure
     *  if isVerticalText() is true, in which case the returned value should
     *  be treated has a height instead of a width.
     *
     *  @param text         The text to be measured
     *  @param length       Number of bytes of text to measure
     *  @param bounds       If not NULL, returns the bounds of the text,
     *                      relative to (0, 0).
     *  @return             The advance width of the text
     */
SK_API float sk_paint_measure_text(sk_paint_t*, const void* text, size_t length, sk_rect_t* bounds);

/** Set the encoding of text
    @param encoding Text encoding
*/
SK_API void sk_paint_set_text_encoding(sk_paint_t*, sk_text_encoding_t encoding);

/** Get the encoding of text
    @return encoding Text encoding
*/
SK_API sk_text_encoding_t sk_paint_get_text_encoding(sk_paint_t*);

SK_C_PLUS_PLUS_END_GUARD

#endif