import AnsiToReact from 'ansi-to-react';

interface TerminalProps {
    logs: string[];
}

export default function Terminal( {logs} : TerminalProps) {
    return (
        <div className="bg-black text-green-400 p-3 rounded-md text-sm font-mono h-[200px] overflow-y-auto">
            {logs.map((log, index) => (
                <div key={index}>
                    <AnsiToReact>
                        {log}
                    </AnsiToReact>
                </div>
            ))}
        </div>
    );
}