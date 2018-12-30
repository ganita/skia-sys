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

#ifndef rs_sk_types_priv_DEFINED
#define rs_sk_types_priv_DEFINED

#include "SkPaint.h"
#include "SkTypeface.h"
#include "rs_sk_types.h"

SK_C_PLUS_PLUS_BEGIN_GUARD

static sk_typeface_t* ToTypeface(SkTypeface* typeface) {
  return reinterpret_cast<sk_typeface_t*>(typeface);
}

static SkTypeface* AsTypeface(sk_typeface_t* ctypeface) {
  return reinterpret_cast<SkTypeface*>(ctypeface);
}

static sk_font_metrics_t* ToFontMetrics(SkFontMetrics* rect) {
  return reinterpret_cast<sk_font_metrics_t*>(rect);
}

static SkFontMetrics* AsFontMetrics(sk_font_metrics_t* cmetrics) {
  return reinterpret_cast<SkFontMetrics*>(cmetrics);
}

static sk_rect_t* ToRect(SkRect* rect) {
  return reinterpret_cast<sk_rect_t*>(rect);
}

static SkRect* AsRect(sk_rect_t* crect) {
  return reinterpret_cast<SkRect*>(crect);
}

static sk_text_encoding_t ToTextEncoding(SkTextEncoding encoding) {
  return (sk_text_encoding_t)(encoding);
}

static SkTextEncoding AsTextEncoding(sk_text_encoding_t cencoding) {
  return (SkTextEncoding)(cencoding);
}

SK_C_PLUS_PLUS_END_GUARD

#endif