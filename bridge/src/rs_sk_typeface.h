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


#ifndef rs_sk_typeface_DEFINED
#define rs_sk_typeface_DEFINED

#include "rs_sk_types.h"

SK_C_PLUS_PLUS_BEGIN_GUARD

/** Return a new typeface given a file. If the file does not exist, or is
    not a valid font file, returns nullptr.
*/
SK_API sk_typeface_t* sk_typeface_new_from_file(const char* path, unsigned int index);

/** Dereference
*/
SK_API void sk_typeface_unref(sk_typeface_t*);

SK_C_PLUS_PLUS_END_GUARD

#endif