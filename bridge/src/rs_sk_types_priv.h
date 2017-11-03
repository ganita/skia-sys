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

#include "rs_sk_types.h"
#include "SkTypeface.h"

SK_C_PLUS_PLUS_BEGIN_GUARD

static sk_typeface_t* ToTypeface(SkTypeface* typeface) {
    return reinterpret_cast<sk_typeface_t*>(typeface);
}

static SkTypeface* AsTypeface(sk_typeface_t* ctypeface) {
    return reinterpret_cast<SkTypeface*>(ctypeface);
}

SK_C_PLUS_PLUS_END_GUARD

#endif