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


#ifndef rs_sk_types_DEFINED
#define rs_sk_types_DEFINED

#include "sk_types.h"

SK_C_PLUS_PLUS_BEGIN_GUARD

/*
    Hold typeface data
 */
typedef struct sk_typeface_t sk_typeface_t;

// Font metrics
typedef struct {
    unsigned int    fFlags;       //!< Bit field to identify which values are unknown
    float           fTop;       //!< The greatest distance above the baseline for any glyph (will be <= 0)
    float           fAscent;    //!< The recommended distance above the baseline (will be <= 0)
    float           fDescent;   //!< The recommended distance below the baseline (will be >= 0)
    float           fBottom;    //!< The greatest distance below the baseline for any glyph (will be >= 0)
    float           fLeading;   //!< The recommended distance to add between lines of text (will be >= 0)
    float           fAvgCharWidth;  //!< the average character width (>= 0)
    float           fMaxCharWidth;  //!< the max character width (>= 0)
    float           fXMin;      //!< The minimum bounding box x value for all glyphs
    float           fXMax;      //!< The maximum bounding box x value for all glyphs
    float           fXHeight;   //!< The height of an 'x' in px, or 0 if no 'x' in face
    float           fCapHeight;  //!< The cap height (> 0), or 0 if cannot be determined.
    float           fUnderlineThickness; //!< underline thickness, or 0 if cannot be determined

    /**  Underline Position - position of the top of the Underline stroke
            relative to the baseline, this can have following values
            - Negative - means underline should be drawn above baseline.
            - Positive - means below baseline.
            - Zero     - mean underline should be drawn on baseline.
     */
    float           fUnderlinePosition; //!< underline position, or 0 if cannot be determined
} sk_font_metrics_t;

SK_C_PLUS_PLUS_END_GUARD

#endif