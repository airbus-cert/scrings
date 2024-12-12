import logging, io
from typing import Any, Dict, Iterable, List, Tuple

from volatility3.framework import interfaces, renderers
from volatility3.framework.configuration import requirements
from volatility3.framework.interfaces import plugins
from volatility3.framework.layers import resources
from volatility3.framework.renderers import format_hints

vollog = logging.getLogger(__name__)

try:
    import pyscrings
    vollog.debug("Using pyscrings module")

except ImportError:
    vollog.info(
        "Unable to find pyscrings:\n\tpip install pyscrings"
    )
    raise


class ScringsScanner(interfaces.layers.ScannerInterface):
    _version = (1, 0, 0)

    def __init__(self, language) -> None:
        super().__init__()
        self.language = language

    def __call__(self, data: bytes, data_offset: int) -> Iterable[Tuple[int, str]]:
        
        for offset, instance in self.language(io.BytesIO(data)):
            yield (
                offset + data_offset,
                instance,
            )
        


class ScringsScan(plugins.PluginInterface):
    """Scans kernel memory using scrings, a semantic scanner base on tree-sitter."""
    _required_framework_version = (2, 4, 0)
    _version = (0, 1, 0)

    @classmethod
    def get_requirements(cls) -> List[interfaces.configuration.RequirementInterface]:
        return cls.get_scrings_option_requirements() + [
            requirements.TranslationLayerRequirement(
                name="primary",
                description="Memory layer for the kernel",
                architectures=["Intel32", "Intel64"],
            )
        ]
        
    @classmethod
    def get_scrings_option_requirements(
        cls,
    ) -> List[interfaces.configuration.RequirementInterface]:
        return [
            requirements.StringRequirement(
                name="language",
                description="Language to match (powershell, bash, javascript, sql, python, php)",
            ),
        ]
        
    @classmethod
    def process_scrings_options(cls, config: Dict[str, Any]):
        language_value = config.get("language")
        if language_value == "powershell":
            return pyscrings.powershell
        elif language_value == "javascript":
            return pyscrings.javascript
        elif language_value == "sql":
            return pyscrings.sql
        elif language_value == "bash":
            return pyscrings.bash
        elif language_value == "python":
            return pyscrings.python
        elif language_value == "php":
            return pyscrings.php
        else:
            vollog.error("Unknown language [%s], must one of the following : [powershell, javascript, sql, bash, python, php]" % language_value)
            return None

    def _generator(self):
        language = self.process_scrings_options(dict(self.config))
        if language is None:
            return None
        layer = self.context.layers[self.config["primary"]]
        for offset, value in layer.scan(context=self.context, scanner=ScringsScanner(language=language)):
            yield 0, (format_hints.Hex(offset), value)

    def run(self):
        return renderers.TreeGrid(
            [
                ("Offset", format_hints.Hex),
                ("Value", str),
            ],
            self._generator(),
        )