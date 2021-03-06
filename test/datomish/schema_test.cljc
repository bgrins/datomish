;; Copyright 2016 Mozilla
;;
;; Licensed under the Apache License, Version 2.0 (the "License"); you may not use
;; this file except in compliance with the License. You may obtain a copy of the
;; License at http://www.apache.org/licenses/LICENSE-2.0
;; Unless required by applicable law or agreed to in writing, software distributed
;; under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
;; CONDITIONS OF ANY KIND, either express or implied. See the License for the
;; specific language governing permissions and limitations under the License.

(ns datomish.schema-test
  (:require
   [datomish.schema :as schema]
   #?@(:clj [[datomish.test-macros :refer [deftest-async]]
             [clojure.test :as t :refer [is are deftest testing]]])
   #?@(:cljs [[datomish.test-macros :refer-macros [deftest-async]]
              [cljs.test :as t :refer-macros [is are deftest testing async]]])))

#?(:clj 
(deftest test-uuid-validation
  (is (not (schema/uuidish? "123e4567-e89b-12d3-a456-426655440000")))
  (is (schema/uuidish? (java.util.UUID/fromString "123e4567-e89b-12d3-a456-426655440000")))))

#?(:cljs
(deftest test-uuid-validation
  ;; Case-insensitive.
  (is (schema/uuidish? "123e4567-e89b-12d3-a456-426655440000"))
  (is (schema/uuidish? "123E4567-e89b-12d3-a456-426655440000"))))
