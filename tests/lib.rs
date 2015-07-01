extern crate assert;
extern crate dwt;

#[test]
fn forward_haar() {
    let mut data = [
        4.217612826262750e-01, 9.157355251890671e-01, 7.922073295595544e-01, 9.594924263929030e-01,
        6.557406991565868e-01, 3.571167857418955e-02, 8.491293058687771e-01, 9.339932477575505e-01,
        6.787351548577735e-01, 7.577401305783334e-01, 7.431324681249162e-01, 3.922270195341682e-01,
        6.554778901775566e-01, 1.711866878115618e-01, 7.060460880196088e-01, 3.183284637742068e-02,
        2.769229849608900e-01, 4.617139063115394e-02, 9.713178123584754e-02, 8.234578283272926e-01,
        6.948286229758170e-01, 3.170994800608605e-01, 9.502220488383549e-01, 3.444608050290876e-02,
        4.387443596563982e-01, 3.815584570930084e-01, 7.655167881490024e-01, 7.951999011370632e-01,
        1.868726045543786e-01, 4.897643957882311e-01, 4.455862007108995e-01, 6.463130101112646e-01,
        7.093648308580726e-01, 7.546866819823609e-01, 2.760250769985784e-01, 6.797026768536748e-01,
        6.550980039738407e-01, 1.626117351946306e-01, 1.189976815583766e-01, 4.983640519821430e-01,
        9.597439585160811e-01, 3.403857266661332e-01, 5.852677509797773e-01, 2.238119394911370e-01,
        7.512670593056529e-01, 2.550951154592691e-01, 5.059570516651424e-01, 6.990767226566860e-01,
        8.909032525357985e-01, 9.592914252054443e-01, 5.472155299638031e-01, 1.386244428286791e-01,
        1.492940055590575e-01, 2.575082541237365e-01, 8.407172559836625e-01, 2.542821789715310e-01,
        8.142848260688164e-01, 2.435249687249893e-01, 9.292636231872278e-01, 3.499837659848087e-01,
        1.965952504312082e-01, 2.510838579760311e-01, 6.160446761466392e-01, 4.732888489027293e-01,
    ];

    let expected_data = [
         4.147168492759418e+00,  1.253279360754873e-01,
         4.084096467714702e-01,  4.658948064130208e-02,
         3.568483024108910e-01, -2.273188749167802e-01,
        -1.164386463345503e-01,  4.094163193731559e-02,
         2.173015620707741e-01,  3.561312405386555e-01,
        -2.661946778248070e-01,  2.165455455832754e-01,
         3.481467793065587e-01, -3.612840951056551e-02,
         3.656565682202418e-01,  2.828584634582116e-01,
        -2.071014740685578e-01, -5.458350879477758e-01,
         1.505578988885113e-01,  4.439282179604442e-02,
        -2.987476169855482e-01,  1.362998684770705e-02,
        -3.702069362683296e-01, -2.076311052397773e-01,
         2.541618794940903e-01,  1.001740028139758e-01,
         2.455249973556499e-01, -9.933579977845319e-02,
         5.821773524743806e-01, -3.440985876361998e-01,
        -1.107187971891155e-01, -3.208272083210646e-01,
        -3.492925366476388e-01, -1.182884263623091e-01,
         4.384267249862666e-01, -6.000786878777276e-02,
        -5.586495407948650e-02,  2.481276222538253e-01,
         3.424455932619815e-01,  4.767407551309557e-01,
         1.631660171201637e-01, -5.135900732507805e-01,
         2.670948384069484e-01,  6.475513972576711e-01,
         4.043653949084614e-02, -2.098913048058437e-02,
        -2.141768395471971e-01, -1.419352880929379e-01,
        -3.204738826591147e-02, -2.854431682706484e-01,
         3.482403802950402e-01, -2.682525330807729e-01,
         4.379524057248081e-01,  2.555878554029040e-01,
         3.508465461282888e-01, -1.365562289386354e-01,
        -4.835774064766307e-02,  2.889175284456297e-01,
        -7.651902898109114e-02,  4.146722196809334e-01,
         4.035881655568866e-01,  4.096127152326055e-01,
        -3.852926389235672e-02,  1.009436134980640e-01,
    ];

    dwt::forward(&mut data, &dwt::wavelet::Haar::new());

    assert::close(&data[..], &expected_data[..], 1e-14);
}

#[test]
fn inverse_haar() {
    let mut data = [
        3.516595070629968e-01, 8.308286278962909e-01, 5.852640911527243e-01, 5.497236082911395e-01,
        9.171936638298100e-01, 2.858390188203735e-01, 7.572002291107213e-01, 7.537290942784953e-01,
        3.804458469753567e-01, 5.678216407252211e-01, 7.585428956306361e-02, 5.395011866660715e-02,
        5.307975530089727e-01, 7.791672301020112e-01, 9.340106842291830e-01, 1.299062084737301e-01,
        5.688236608721927e-01, 4.693906410582058e-01, 1.190206950124140e-02, 3.371226443988815e-01,
        1.621823081932428e-01, 7.942845406839070e-01, 3.112150420448049e-01, 5.285331355062127e-01,
        1.656487294997809e-01, 6.019819414016365e-01, 2.629712845401443e-01, 6.540790984767823e-01,
        6.892145031400078e-01, 7.481515928237095e-01, 4.505415985024978e-01, 8.382137799693257e-02,
        2.289769687168188e-01, 9.133373615016696e-01, 1.523780189692230e-01, 8.258169774895474e-01,
        5.383424352600571e-01, 9.961347166268855e-01, 7.817552875318368e-02, 4.426782697754463e-01,
        1.066527701805844e-01, 9.618980808550537e-01, 4.634224134067444e-03, 7.749104647115024e-01,
        8.173032206534330e-01, 8.686947053635097e-01, 8.443584551091032e-02, 3.997826490988965e-01,
        2.598704028506542e-01, 8.000684802243075e-01, 4.314138274635446e-01, 9.106475944295229e-01,
        1.818470283028525e-01, 2.638029165219901e-01, 1.455389803847170e-01, 1.360685587086637e-01,
        8.692922076400893e-01, 5.797045873655702e-01, 5.498602018363320e-01, 1.449547982237268e-01,
        8.530311177218937e-01, 6.220551314850660e-01, 3.509523808922709e-01, 5.132495398670534e-01,
    ];

    let expected_data = [
         1.061401401623698e+00,  7.375790669932929e-01,
         9.764936152651623e-01, -3.151604683925572e-01,
         6.885054166478777e-01,  4.730103556140577e-01,
         6.953080298745993e-01, -4.725735397290757e-01,
         6.093455404763515e-01, -1.519856326693813e-01,
         9.211514975044544e-01, -4.875957286999670e-01,
         4.505815519524989e-02, -6.549873781319032e-02,
        -3.432212926570571e-02, -6.603637421499977e-01,
         2.991343120549443e-01,  1.483045180009010e-01,
         7.417017626176143e-01, -6.186275489482547e-01,
         4.894103400518620e-01,  4.828565574303871e-01,
         2.397933524671397e-01, -8.560955363527047e-01,
         7.254926282645878e-01, -4.303486709547074e-01,
         4.506168535535229e-01, -7.779029803332524e-01,
         2.777876895666517e-01,  1.583773716946829e-01,
        -2.776158269699375e-02, -5.931396270540972e-01,
         6.808280033939725e-01,  3.133157552232174e-01,
         8.971569975890371e-01, -2.343106979714090e-01,
         6.449635789867678e-01,  3.485229319253658e-02,
         3.818510839803482e-01, -9.059990946043170e-01,
         3.835302505817852e-01,  1.263597168786472e-01,
         1.785105303595598e-01, -1.945631319794160e-01,
         2.456058572073344e-03, -2.033671413419453e-01,
        -6.583196392925423e-01, -8.507496404308943e-01,
         1.320870121349718e+00,  9.150529163985532e-02,
         4.268862481659231e-01, -3.929398414563651e-01,
         4.640208402405691e-01, -3.135989146055791e-01,
        -5.704421092166897e-01, -7.754391507957396e-01,
         5.288769311465078e-01, -6.774912446620744e-01,
        -8.498935351530146e-02, -9.647081570052607e-01,
        -1.013640195380850e-01, -5.976856363430628e-01,
        -7.042397585663779e-02, -7.962684360183752e-01,
    ];

    dwt::inverse(&mut data, &dwt::wavelet::Haar::new());

    assert::close(&data[..], &expected_data[..], 1e-14);
}
