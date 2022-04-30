import gettext
import os
import sys
import threading

from jinja2 import pass_context
from jinja2.ext import InternationalizationExtension
from markupsafe import Markup

localedir = os.path.join("..", "locales")
domain = "messages"
threadLocalData = threading.local()
threadLocalData.locale = "en_US"

locales = []
for dirpath, dirnames, filenames in os.walk(localedir):
    for dirname in dirnames:
        locales.append(dirname)
    break

all_translations = {}
for locale in locales:
    all_translations[locale] = gettext.translation(domain, localedir, [locale])


def context_locale(context):
    lang = "en_US"
    if "locale" in context and context["locale"] in all_translations:
        lang = context["locale"]
    return lang


def parseAcceptLanguage(acceptLanguage):
    try:
        languages = acceptLanguage.split(",")
        locale_q_pairs = []

        for language in languages:
            if language.split(";")[0] == language:
                # no q => q = 1
                locale_q_pairs.append((language.strip().replace("-", "_"), "1"))
            else:
                locale = language.split(";")[0].strip()
                q = language.split(";")[1].split("=")[1]
                locale_q_pairs.append((locale.replace("-", "_"), q))

        return locale_q_pairs
    except AttributeError:
        return "en_US"


def setLocale(locale):
    languagePairs = parseAcceptLanguage(locale)
    for pair in languagePairs:
        if pair[0] in locales:
            threadLocalData.locale = pair[0]
            break


class InternationalizationWithContextExtension(InternationalizationExtension):
    def _install_callables(self, gettext, ngettext, newstyle=None):
        if newstyle is not None:
            self.environment.newstyle_gettext = newstyle
        if self.environment.newstyle_gettext:
            gettext = _make_new_gettext(gettext)
            ngettext = _make_new_ngettext(ngettext)
        self.environment.globals.update(gettext=gettext, ngettext=ngettext)

    def gettext(msg):
        return all_translations[threadLocalData.locale].gettext(msg)

    def ngettext(singular, plural, n):
        return all_translations[threadLocalData.locale].ngettext(singular, plural, n)


@pass_context
def _gettext_alias(__context, *args, **kwargs):
    return __context.call(__context.resolve("gettext"), *args, **kwargs)


def _make_new_gettext(func):
    @pass_context
    def gettext(__context, __string, **variables):
        rv = __context.call(func, __context, __string)
        if __context.eval_ctx.autoescape:
            rv = Markup(rv)
        return rv % variables

    return gettext


def _make_new_ngettext(func):
    @pass_context
    def ngettext(__context, __singular, __plural, __num, **variables):
        variables.setdefault("num", __num)
        rv = __context.call(func, __context, __singular, __plural, __num)
        if __context.eval_ctx.autoescape:
            rv = Markup(rv)
        return rv % variables

    return ngettext


i18n = InternationalizationWithContextExtension
