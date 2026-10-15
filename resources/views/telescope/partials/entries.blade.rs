<div class="overflow-x-auto">
    <table class="w-full text-left border-collapse">
        <thead>
            <tr class="border-b border-slate-800 bg-slate-800/30">
                <th class="px-6 py-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">Time</th>
                <th class="px-6 py-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">Type</th>
                <th class="px-6 py-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">Content</th>
                <th class="px-6 py-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">Status</th>
            </tr>
        </thead>
        <tbody class="divide-y divide-slate-800">
            {% if entries %}
                {% for entry in entries %}
                <tr class="hover:bg-slate-800/50 transition-colors group">
                    <td class="px-6 py-4 text-sm text-slate-400 whitespace-nowrap">
                        {{ entry.created_at | date(format="%H:%M:%S") }}
                    </td>
                    <td class="px-6 py-4">
                        <span class="px-2 py-1 text-[10px] font-bold rounded uppercase tracking-tighter
                            {% if entry.entry_type == 'Request' %}bg-sky-500/10 text-sky-400
                            {% elif entry.entry_type == 'Event' %}bg-purple-500/10 text-purple-400
                            {% elif entry.entry_type == 'Job' %}bg-amber-500/10 text-amber-400
                            {% else %}bg-slate-500/10 text-slate-400{% endif %}">
                            {{ entry.entry_type }}
                        </span>
                    </td>
                    <td class="px-6 py-4">
                        <div class="flex flex-col gap-1">
                            {% if entry.entry_type == 'Request' %}
                                <div class="flex items-center gap-2">
                                    <span class="font-bold text-xs {{ entry.content.method | lower }}">{{ entry.content.method }}</span>
                                    <span class="text-sm text-slate-300 fira truncate max-w-md">{{ entry.content.uri }}</span>
                                </div>
                                <span class="text-[11px] text-slate-500">{{ entry.content.ip }} • {{ entry.content.duration_ms }}ms</span>
                            {% elif entry.entry_type == 'Event' %}
                                <span class="text-sm font-semibold text-slate-200">{{ entry.content.name }}</span>
                                <span class="text-[11px] text-slate-500 fira truncate max-w-md">{{ entry.content.payload }}</span>
                            {% else %}
                                <span class="text-sm text-slate-300">{{ entry.content | json_encode }}</span>
                            {% endif %}
                        </div>
                    </td>
                    <td class="px-6 py-4">
                        {% if entry.entry_type == 'Request' %}
                            <span class="text-sm font-medium {% if entry.content.status >= 400 %}text-rose-400{% else %}text-emerald-400{% endif %}">
                                {{ entry.content.status }}
                            </span>
                        {% else %}
                            <span class="text-slate-500 text-xs">—</span>
                        {% endif %}
                    </td>
                </tr>
                {% endfor %}
            {% else %}
                <tr>
                    <td colspan="4" class="px-6 py-20 text-center text-slate-500">
                        <div class="flex flex-col items-center gap-2">
                            <svg class="w-12 h-12 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path></svg>
                            <p>No entries found for <b>{{ active_tag }}</b></p>
                        </div>
                    </td>
                </tr>
            {% endif %}
        </tbody>
    </table>
</div>

<style>
    .get { color: #10b981; }
    .post { color: #3b82f6; }
    .put { color: #f59e0b; }
    .delete { color: #ef4444; }
</style>
