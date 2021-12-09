using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using UnityEngine;
using UnityEngine.Profiling;

public class ServiceBell : MonoBehaviour {
	void Start () {
		var connection = FFI.connectToServer("127.0.0.1:12345");

		UnityEngine.Debug.Log("Tried to connect " + connection);

		Stopwatch watch = new Stopwatch();
		watch.Start();

		double res = ProcessNums();
		watch.Stop();

		long ticksRes1 = watch.ElapsedTicks;

		
		watch.Restart();
		double res2 = FFI.ProcessHeavy();
		watch.Stop();

		long ticksRes2 = watch.ElapsedTicks;

		UnityEngine.Debug.Log($"Results\nC# {ticksRes1}\nRust {ticksRes2}");

	}

	void OnDestroy () {
		FFI.disconnectFromServer();
	}

	void Update () {
		if (Input.GetMouseButtonDown(0)) {
			Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
			RaycastHit hit;
			if (Physics.Raycast(ray, out hit)) {
				gameObject.GetComponent<AudioSource>().Play();
				var result = FFI.sendDing();

				UnityEngine.Debug.Log("Send Ding - Got " + result);
			}
		}
	}

	double ProcessNums()
	{
		double total = 0;
		for(int i = 0; i < 1_000_000; i++)
		{
			double div = 1.0 / (double)i;
			total += div;
		}

		return total;
	}
}
