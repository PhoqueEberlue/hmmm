import { Hypercube } from "wasm-hmmm";

function display_hypercube(hypercube) {
    let nb_proc_dim = hypercube.number_processor_per_dimension;
    let x_list = [];    
    let y_list = [];    
    let z_list = [];
    let colors_a = [];
    let colors_b = [];
    let colors_c = [];

    for (let y = 0; y < nb_proc_dim; y++) {
        for (let x = 0; x < nb_proc_dim; x++) {
            for (let z = 0; z  < nb_proc_dim; z++) {
                x_list.push(x);
                y_list.push(y);
                z_list.push(z);

                let processor = hypercube.get_processor_copy(x, y, z);
                colors_a.push(`rgb(${processor.a*55n}, 0, 0)`);
                colors_b.push(`rgb(0, ${processor.b*55n}, 0)`);
                colors_c.push(`rgb(0, 0, ${processor.c*55n})`);
            }
        }
    }

    let plot_a = document.getElementById('plot_a');
    let plot_b = document.getElementById('plot_b');
    let plot_c = document.getElementById('plot_c');

    let trace_a = {
        x: x_list, y: y_list, z: z_list,
        mode: 'markers',
        marker: {
            color: colors_a,
            size: 12,
            symbol: 'circle',
            line: {
                color: colors_a,
                width: 1
            },
            opacity: 0.9
        },
        type: 'scatter3d'
    };

    let trace_b = {
        x: x_list, y: y_list, z: z_list,
        mode: 'markers',
        marker: {
            color: colors_b,
            size: 12,
            symbol: 'circle',
            line: {
                color: colors_b,
                width: 1
            },
            opacity: 0.9
        },
        type: 'scatter3d'
    };

    let trace_c = {
        x: x_list, y: y_list, z: z_list,
        mode: 'markers',
        marker: {
            color: colors_c,
            size: 12,
            symbol: 'circle',
            line: {
                color: colors_c,
                width: 1
            },
            opacity: 0.9
        },
        type: 'scatter3d'
    };

    let layout = {
        margin: {
            l: 0,
            r: 0,
            b: 0,
            t: 0
        },
        scene: {
            camera: {
                up: {
                    x: 0,
                    y: 1,
                    z: 0
                },
                eye: {
                    x: 0.9,
                    y: 1.1,
                    z: 1.25,
                },
            }
        }

    };

    Plotly.newPlot(plot_a, [trace_a], layout);
    Plotly.newPlot(plot_b, [trace_b], layout);
    Plotly.newPlot(plot_c, [trace_c], layout);

    let repr_a = document.getElementById('repr_a');
    let repr_b = document.getElementById('repr_b');
    let repr_c = document.getElementById('repr_c');

    repr_a.textContent = hypercube.repr_register("a");
    repr_b.textContent = hypercube.repr_register("b");
    repr_c.textContent = hypercube.repr_register("c");

}

const hypercube = Hypercube.new(4);
hypercube.init(
    [1n,2n,0n,0n, 
     0n,1n,1n,3n,
     1n,0n,0n,2n,
     0n,0n,2n,0n],
    [1n,2n,2n,0n,  
     0n,3n,2n,0n, 
     1n,0n,1n,2n,    
     0n,1n,2n,0n]
)

const nextStepButton = document.getElementById("next_step_button");

display_hypercube(hypercube);

nextStepButton.addEventListener("click", event => {
    if (hypercube.next_step()) {
        display_hypercube(hypercube);
    }
});
