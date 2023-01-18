/*
 *  Copyright 2023 Tesseract Systems, Inc.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */
package one.tesseract.devwallet.ui.util

import androidx.annotation.MainThread
import androidx.lifecycle.MutableLiveData

/**
 * Used for cases where T is Unit, to make calls cleaner.
 */
@MainThread
fun MutableLiveData<Unit>.call() {
    value = Unit
}