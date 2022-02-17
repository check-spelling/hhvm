/*
   +----------------------------------------------------------------------+
   | HipHop for PHP                                                       |
   +----------------------------------------------------------------------+
   | Copyright (c) 2010-present Facebook, Inc. (http://www.facebook.com)  |
   +----------------------------------------------------------------------+
   | This source file is subject to version 3.01 of the PHP license,      |
   | that is bundled with this package in the file LICENSE, and is        |
   | available through the world-wide-web at the following url:           |
   | http://www.php.net/license/3_01.txt                                  |
   | If you did not receive a copy of the PHP license and are unable to   |
   | obtain it through the world-wide-web, please send a note to          |
   | license@php.net so we can mail you a copy immediately.               |
   +----------------------------------------------------------------------+
*/

#include "hphp/runtime/base/autoload-handler.h"
#include "hphp/runtime/vm/decl-provider.h"

namespace HPHP {

  DeclProviderResult HhvmDeclProvider::getDecl(AutoloadMap::KindOf /*kind*/, char const* /*symbol*/) {
  // The prototypical, indicative stub code sketched out below doesn't
  // take into account the RDS nature of the AutoloadHandler data
  // structures and at this time, we can't rely on calls to `getDecl`
  // not triggering assertions due to inappropriate usage (like being
  // called on the wrong thread for example). See D31842297 for some
  // discussion.

    return DeclProviderResult{DeclProviderResult::Tag::Missing, {}};

  /*
  String sym = String(symbol, CopyStringMode::CopyString);
  Optional<String> filename_opt = AutoloadHandler::s_instance->getFile(symbol, kind);
  if (filename_opt.has_value()) {
    String filename = filename_opt.value();
    auto result = m_cache.find(filename.data());

    if (result != m_cache.end()) {
      return &(*result->second.decls);
    }

    std::ifstream s(filename.data());
    std::string text {
      std::istreambuf_iterator<char>(s), std::istreambuf_iterator<char>() };

    DeclResult decl_result = hackc_direct_decl_parse(*opts, filename.toCppString(), text);

    m_cache.insert({filename.data(), std::move(decl_result)});
    return &*decl_result.decls;
  } else {
    return nullptr;
  }
  */
}

extern "C" {
  DeclProviderResult hhvm_decl_provider_get_decl(void* provider, int sort, char const* symbol) {
    try {
      // Unsafe: if `sort` is out of range the result of this cast is
      // UB.
      HPHP::AutoloadMap::KindOf kind {
        static_cast<HPHP::AutoloadMap::KindOf>(sort)
      };

      return ((HhvmDeclProvider*)provider)->getDecl(kind, symbol);
    }
    catch(...) {
    }

    return DeclProviderResult{DeclProviderResult::Tag::Missing, {}};
  }

} //extern "C"
}//namespace HPHP
