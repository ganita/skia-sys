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


#include "skia/include/c/sk_canvas.h"
#include "skia/include/c/sk_data.h"
#include "skia/include/c/sk_image.h"
#include "skia/include/c/sk_maskfilter.h"
#include "skia/include/c/sk_matrix.h"
#include "skia/include/c/sk_paint.h"
#include "skia/include/c/sk_path.h"
#include "skia/include/c/sk_picture.h"
#include "skia/include/c/sk_shader.h"
#include "skia/include/c/sk_surface.h"
#include "skia/include/c/sk_types.h"
#include "bridge/src/rs_sk_canvas.h"
#include "bridge/src/rs_sk_paint.h"
#include "bridge/src/rs_sk_typeface.h"