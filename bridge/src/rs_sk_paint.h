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

SK_C_PLUS_PLUS_END_GUARD

#endif