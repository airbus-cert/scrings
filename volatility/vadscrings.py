import logging
from typing import Iterable, List, Tuple

from volatility3.framework import interfaces, renderers
from volatility3.framework.configuration import requirements
from volatility3.framework.renderers import format_hints
from .scrings import ScringsScanner, ScringsScan
from volatility3.plugins.windows import pslist

vollog = logging.getLogger(__name__)


class VadScringsScan(interfaces.plugins.PluginInterface):
    """Scans all the Virtual Address Descriptor memory maps using scrings, a semantic scanner base on tree-sitter."""

    _required_framework_version = (2, 4, 0)
    _version = (0, 1, 0)

    @classmethod
    def get_requirements(cls) -> List[interfaces.configuration.RequirementInterface]:
        vadscrings_requirements = [
            requirements.ModuleRequirement(
                name="kernel",
                description="Windows kernel",
                architectures=["Intel32", "Intel64"],
            ),
            requirements.PluginRequirement(
                name="pslist", plugin=pslist.PsList, version=(2, 0, 0)
            ),
            requirements.ListRequirement(
                name="pid",
                element_type=int,
                description="Process IDs to include (all other processes are excluded)",
                optional=True,
            ),
        ]

        scrings_requirements = ScringsScan.get_scrings_option_requirements()

        # return the combined requirements
        return scrings_requirements + vadscrings_requirements

    def _generator(self):
        kernel = self.context.modules[self.config["kernel"]]

        language = ScringsScan.process_scrings_options(dict(self.config))

        filter_func = pslist.PsList.create_pid_filter(self.config.get("pid", None))

        sanity_check = 1024 * 1024 * 1024  # 1 GB

        for task in pslist.PsList.list_processes(
            context=self.context,
            layer_name=kernel.layer_name,
            symbol_table=kernel.symbol_table_name,
            filter_func=filter_func,
        ):
            layer_name = task.add_process_layer()
            layer = self.context.layers[layer_name]

            max_vad_size = 0
            vad_maps_to_scan = []

            for start, size in self.get_vad_maps(task):
                if size > sanity_check:
                    vollog.debug(
                        f"VAD at 0x{start:x} over sanity-check size, not scanning"
                    )
                    continue
                max_vad_size = max(max_vad_size, size)
                vad_maps_to_scan.append((start, size))

            if not vad_maps_to_scan:
                vollog.warning(
                    f"No VADs were found for task {task.UniqueProcessId}, not scanning"
                )
                continue

            scanner = ScringsScanner(language=language)
            scanner.chunk_size = max_vad_size

            # scan the VAD data (in one contiguous block) with the scringsscanner
            for start, size in vad_maps_to_scan:
                for offset, value in scanner(
                    layer.read(start, size, pad=True), start
                ):
                    yield 0, (
                        format_hints.Hex(offset),
                        task.UniqueProcessId,
                        repr(value),
                    )

    @staticmethod
    def get_vad_maps(
        task: interfaces.objects.ObjectInterface,
    ) -> Iterable[Tuple[int, int]]:
        """Creates a map of start/end addresses within a virtual address
        descriptor tree.

        Args:
            task: The EPROCESS object of which to traverse the vad tree

        Returns:
            An iterable of tuples containing start and size for each descriptor
        """
        vad_root = task.get_vad_root()
        for vad in vad_root.traverse():
            yield (vad.get_start(), vad.get_size())

    def run(self):
        return renderers.TreeGrid(
            [
                ("Offset", format_hints.Hex),
                ("PID", int),
                ("Value", str),
            ],
            self._generator(),
        )