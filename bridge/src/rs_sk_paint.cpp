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


#include "SkPaint.h"
#include "rs_sk_paint.h"
#include "sk_types_priv.h"
#include "rs_sk_types_priv.h"

void sk_paint_set_dither(sk_paint_t* cpaint, bool dither) {
    AsPaint(cpaint)->setAntiAlias(dither);
}

bool sk_paint_is_dither(sk_paint_t* cpaint) {
    return AsPaint(cpaint)->isAntiAlias();
}

void sk_paint_set_text_size(sk_paint_t* cpaint, float size) {
    AsPaint(cpaint)->setTextSize(size);
}

float sk_paint_get_text_size(sk_paint_t* cpaint) {
    return AsPaint(cpaint)->getTextSize();
}

void sk_paint_set_typeface(sk_paint_t* cpaint, sk_typeface_t* ctypeface) {
    AsPaint(cpaint)->setTypeface(sk_ref_sp(AsTypeface(ctypeface)));
}

float sk_paint_get_font_metrics(sk_paint_t* cpaint, sk_font_metrics_t* cmetrics, float scale) {
    return AsPaint(cpaint)->getFontMetrics(AsFontMetrics(cmetrics), scale);
}